export type WASIOptions = {
    version: "preview1";
    stdin?: number | FD;
    stdout?: number | FD;
    stderr?: number | FD;
    args?: string[];
    env?: Record<string, string | undefined>;
    preopens?: Record<string, string>;
    returnOnExit?: boolean;
    trace?: boolean;
    sysCalls: WASISysCalls;
};

export type WASISysCalls = {
    processExit(exitCode: number): never;
    pathOpen(path: string, mode: "r" | "w"): number;
    fdRead(fd: number, buffer: Uint8Array): number;
    fdWrite(fd: number, buffer: Uint8Array): number;
    fdClose(fd: number): void;
};

export type FD = {
    getPreopenDirName(): string;
    openChild(fileName: string, mode: "r" | "w"): { code: number; fd?: FD };
    read(buffer: Uint8Array): { code: number; nread: number };
    write(buffer: Uint8Array): { code: number; nwritten: number };
    close(): number;
};

const WASI_ERRNO_SUCCESS = 0;
const WASI_ERRNO_BADF = 8;
const WASI_ERRNO_NOENT = 2;
const WASI_ERRNO_INVAL = 28;

const WASI_FILETYPE_DIRECTORY = 3;
const WASI_FILETYPE_REGULAR_FILE = 4;

class SysFD implements FD {
    sysFileFd: number | null;
    sysDirName: string;
    sysCalls: WASISysCalls;

    constructor(
        sysFileFd: number | null,
        sysDirName: string,
        sysCalls: WASISysCalls
    ) {
        this.sysFileFd = sysFileFd;
        this.sysDirName = sysDirName;
        this.sysCalls = sysCalls;
    }

    getPreopenDirName() {
        return this.sysDirName;
    }

    openChild(fileName: string, mode: "r" | "w") {
        const fullPath = `${this.sysDirName}/${fileName}`;
        try {
            const sysFd = this.sysCalls.pathOpen(fullPath, mode);
            const fd = new SysFD(sysFd, fullPath, this.sysCalls);

            return { code: WASI_ERRNO_SUCCESS, fd };
        } catch {
            return { code: WASI_ERRNO_NOENT, fd: undefined };
        }
    }

    read(buffer: Uint8Array) {
        if (this.sysFileFd === null) {
            return { code: WASI_ERRNO_INVAL, nread: 0 };
        }

        try {
            const bytesRead = this.sysCalls.fdRead(this.sysFileFd, buffer);
            return { code: WASI_ERRNO_SUCCESS, nread: bytesRead };
        } catch {
            return { code: WASI_ERRNO_INVAL, nread: 0 };
        }
    }

    write(buffer: Uint8Array) {
        if (this.sysFileFd === null) {
            return { code: WASI_ERRNO_INVAL, nwritten: 0 };
        }

        try {
            const bytesRead = this.sysCalls.fdWrite(this.sysFileFd, buffer);
            return { code: WASI_ERRNO_SUCCESS, nwritten: bytesRead };
        } catch {
            return { code: WASI_ERRNO_INVAL, nwritten: 0 };
        }
    }

    close() {
        if (this.sysFileFd === null || this.sysFileFd < 3) {
            return WASI_ERRNO_SUCCESS;
        }

        try {
            this.sysCalls.fdClose(this.sysFileFd);
            return WASI_ERRNO_SUCCESS;
        } catch {
            return WASI_ERRNO_INVAL;
        }
    }
}

class VirtualFD implements FD {
    contents: Uint8Array;
    readCursor: number;
    writes: Uint8Array[];

    constructor() {
        this.contents = new Uint8Array();
        this.readCursor = 0;
        this.writes = [];
    }

    read(buffer: Uint8Array) {
        if (this.readCursor >= this.contents.length - 1) {
            return { code: WASI_ERRNO_SUCCESS, nread: 0 };
        }

        const bytesRead = Math.min(
            buffer.length,
            this.contents.length - this.readCursor
        );
        buffer.set(
            this.contents.subarray(this.readCursor, this.readCursor + bytesRead)
        );

        this.readCursor += bytesRead;

        return { code: WASI_ERRNO_SUCCESS, nread: bytesRead };
    }

    write(buffer: Uint8Array) {
        this.writes.push(buffer.slice());
        return { code: WASI_ERRNO_SUCCESS, nwritten: buffer.length };
    }

    flushAndRead() {
        this.flush();
        return this.contents;
    }

    flushAndReadUtf8() {
        this.flush();
        return new TextDecoder().decode(this.contents);
    }

    flush() {
        if (this.writes.length === 0) {
            return;
        }

        const newSize = this.writes.reduce((acc, w) => acc + w.length, 0);
        this.contents = new Uint8Array(newSize);

        let offset = 0;
        for (let i = 0; i < this.writes.length; i++) {
            const write = this.writes[i];
            this.contents.set(write, offset);
            offset += write.length;
        }

        this.writes = [];
    }

    close() {
        return WASI_ERRNO_SUCCESS;
    }

    getPreopenDirName(): never {
        throw new Error("Not supported");
    }

    openChild(): never {
        throw new Error("Not supported");
    }
}

export class WASI {
    static FD = SysFD;
    static VirtualFD = VirtualFD;

    options: WASIOptions;
    args: string[];
    nextFd: number;
    memory: WebAssembly.Memory;
    returnOnExit: boolean;
    sysCalls: WASISysCalls;
    fds: FD[];
    preopenCount: number;

    constructor(options: WASIOptions) {
        this.options = options;
        this.args = this.options.args ?? [];
        this.nextFd = 3;
        this.memory = void 0 as never;
        this.returnOnExit = this.options.returnOnExit ?? true;
        this.sysCalls = this.options.sysCalls;
        this.fds = [];

        if (typeof this.options.stdin === "object") {
            this.fds.push(this.options.stdin);
        } else {
            this.fds.push(
                new SysFD(this.options.stdin ?? 0, "<stdin>", this.sysCalls)
            );
        }

        if (typeof this.options.stdout === "object") {
            this.fds.push(this.options.stdout);
        } else {
            this.fds.push(
                new SysFD(this.options.stdout ?? 1, "<stdout>", this.sysCalls)
            );
        }

        if (typeof this.options.stderr === "object") {
            this.fds.push(this.options.stderr);
        } else {
            this.fds.push(
                new SysFD(this.options.stderr ?? 2, "<stderr>", this.sysCalls)
            );
        }

        for (const path of Object.values(this.options.preopens ?? {})) {
            this.fds.push(new SysFD(null, path, this.sysCalls));
        }

        this.preopenCount = this.fds.length;
    }

    static async NodeSysCalls(): Promise<WASISysCalls> {
        const fs = await import("node:fs");
        const process = await import("node:process");

        return {
            processExit: (exitCode) => process.exit(exitCode),
            pathOpen: (path, mode) => fs.openSync(path, mode),
            fdRead: (fd, buffer) => fs.readSync(fd, buffer),
            fdWrite: (fd, buffer) => fs.writeSync(fd, buffer),
            fdClose: (fd) => {
                try {
                    fs.closeSync(fd);
                } catch (err) {
                    console.error(err);
                    process.exit(1);
                }
            },
        };
    }

    static DenoSysCalls(): WASISysCalls {
        const files: Map<number, Deno.FsFile> = new Map();

        const getFile = (fd: number) => {
            if (fd === 0) {
                return Deno.stdin as never;
            } else if (fd === 1) {
                return Deno.stdout as never;
            } else if (fd === 2) {
                return Deno.stderr as never;
            }

            const file = files.get(fd);
            if (file === undefined) {
                throw new Error(`Unknown fd: ${fd}`);
            }

            return file;
        };

        return {
            processExit: (exitCode) => Deno.exit(exitCode),
            pathOpen: (path, mode) => {
                const file = Deno.openSync(
                    path,
                    mode === "r" ? { read: true } : { write: true }
                );

                const fd = files.size + 3;
                files.set(fd, file);
                return fd;
            },
            fdRead: (fd, buffer) => {
                const file = getFile(fd);
                return file.readSync(buffer) ?? 0;
            },
            fdWrite: (fd, buffer) => {
                const file = getFile(fd);
                return file.writeSync(buffer) ?? 0;
            },
            fdClose: (fd) => {
                const file = getFile(fd);
                return file.close();
            },
        };
    }

    start(instance: WebAssembly.Instance): number {
        this.memory = instance.exports.memory as WebAssembly.Memory;

        let exitCode = 0;

        try {
            if (typeof instance.exports._start !== "function") {
                throw new Error("_start function is not exported");
            }

            instance.exports._start();
        } catch (err) {
            if (!(err instanceof ProcExitError)) {
                throw err;
            }

            exitCode = err.exitCode;
        } finally {
            for (let fd = this.preopenCount; fd < this.fds.length; fd++) {
                this.fds[fd].close();
            }
        }

        if (!this.returnOnExit) {
            this.sysCalls.processExit(exitCode);
        }

        return exitCode;
    }

    getImportObject(): WebAssembly.Imports {
        const imports = this.#getImportObject();
        if (!this.options.trace) {
            return imports;
        }

        for (const [fnName, fn] of Object.entries(
            imports.wasi_snapshot_preview1
        )) {
            imports.wasi_snapshot_preview1[fnName] = (...args: unknown[]) => {
                console.log(`[WASI] ${fnName}(${args.join(", ")})`);

                // @ts-ignore: fn is a function
                return fn(...args);
            };
        }

        return imports;
    }

    #getImportObject(): WebAssembly.Imports {
        return {
            wasi_snapshot_preview1: {
                path_open: (
                    dirfd: number,
                    _dirflags: number,
                    path_ptr: number,
                    path_len: number,
                    _oflags: number,
                    _fs_rights_base: number,
                    _fs_rights_inheriting: number,
                    _fdflags: number,
                    fd_ptr: number
                ) => {
                    const dir = this.fds[dirfd];
                    if (dir === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const pathBytes = new Uint8Array(
                        this.memory.buffer,
                        path_ptr,
                        path_len
                    );
                    const path = new TextDecoder()
                        .decode(pathBytes)
                        .replace(/\0/g, "");

                    const { code: err, fd: childFile } = dir.openChild(
                        path,
                        "r"
                    );
                    if (childFile !== undefined) {
                        this.fds.push(childFile);

                        const childFd = this.fds.length - 1;
                        const memory = new DataView(this.memory.buffer);
                        memory.setUint32(fd_ptr, childFd, true);
                    }

                    return err;
                },
                fd_read: (
                    fd: number,
                    iovs_ptr: number,
                    iovs_len: number,
                    nread_ptr: number
                ) => {
                    const file = this.fds[fd];
                    if (file === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    let totalBytesRead = 0;

                    const memory = new DataView(this.memory.buffer);

                    for (let i = 0; i < iovs_len; i++) {
                        const baseOffset = iovs_ptr + i * 8;
                        const bufPtr = memory.getUint32(baseOffset, true);
                        const bufLen = memory.getUint32(baseOffset + 4, true);

                        const buffer = new Uint8Array(
                            this.memory.buffer,
                            bufPtr,
                            bufLen
                        );

                        const { code, nread } = file.read(buffer);
                        if (code != WASI_ERRNO_SUCCESS) {
                            return code;
                        }

                        totalBytesRead += nread;

                        if (nread < bufLen) {
                            break;
                        }
                    }

                    memory.setUint32(nread_ptr, totalBytesRead, true);
                    return WASI_ERRNO_SUCCESS;
                },
                fd_write: (
                    fd: number,
                    iovs_ptr: number,
                    iovs_len: number,
                    nwritten_ptr: number
                ) => {
                    const file = this.fds[fd];
                    if (file === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    let totalBytesWritten = 0;

                    const memory = new DataView(this.memory.buffer);

                    for (let i = 0; i < iovs_len; i++) {
                        const baseOffset = iovs_ptr + i * 8;
                        const bufPtr = memory.getUint32(baseOffset, true);
                        const bufLen = memory.getUint32(baseOffset + 4, true);

                        const buffer = new Uint8Array(
                            this.memory.buffer,
                            bufPtr,
                            bufLen
                        );

                        const { code, nwritten } = file.write(buffer);
                        if (code != WASI_ERRNO_SUCCESS) {
                            return code;
                        }
                        totalBytesWritten += nwritten;

                        if (nwritten < bufLen) {
                            break;
                        }
                    }

                    memory.setUint32(nwritten_ptr, totalBytesWritten, true);
                    return WASI_ERRNO_SUCCESS;
                },
                fd_seek: () => {
                    throw new Error("Not Implemented");
                },
                fd_close: (fd: number) => {
                    // don't allow closing preopens
                    if (fd < this.preopenCount) {
                        return WASI_ERRNO_INVAL;
                    }

                    const file = this.fds[fd];
                    if (file === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const code = file.close();

                    if (code === WASI_ERRNO_SUCCESS) {
                        this.fds.splice(fd, 1);
                    }

                    return code;
                },
                args_sizes_get: (
                    argc_ptr: number,
                    argv_buf_size_ptr: number
                ) => {
                    const memory = new DataView(this.memory.buffer);

                    const argc = this.args.length;
                    memory.setUint32(argc_ptr, argc, true);

                    const argvBufferSize = this.args.reduce(
                        (acc, arg) => acc + arg.length + 1,
                        0
                    );
                    memory.setUint32(argv_buf_size_ptr, argvBufferSize, true);

                    return WASI_ERRNO_SUCCESS;
                },
                args_get: (argv_ptr: number, argv_buf_ptr: number) => {
                    const memory = new DataView(this.memory.buffer);

                    const encoder = new TextEncoder();
                    let bufferOffset = argv_buf_ptr;

                    for (const [i, arg] of Object.entries(this.args)) {
                        const argBytes = encoder.encode(arg + "\0");
                        new Uint8Array(
                            this.memory.buffer,
                            bufferOffset,
                            argBytes.length
                        ).set(argBytes);
                        memory.setUint32(
                            argv_ptr + Number(i) * 4,
                            bufferOffset,
                            true
                        );
                        bufferOffset += argBytes.length;
                    }

                    return WASI_ERRNO_SUCCESS;
                },
                proc_exit: (exit_code: number) => {
                    throw new ProcExitError(exit_code);
                },
                fd_prestat_get: (fd: number, buf_ptr: number) => {
                    // don't allow touching non-preopens
                    if (fd >= this.preopenCount) {
                        return WASI_ERRNO_INVAL;
                    }

                    const dir = this.fds[fd];
                    if (dir === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const memory = new DataView(this.memory.buffer);

                    memory.setUint8(buf_ptr, 0); // dir

                    const dirName = dir.getPreopenDirName();
                    memory.setUint32(buf_ptr + 4, dirName.length, true);

                    return WASI_ERRNO_SUCCESS;
                },
                fd_prestat_dir_name: (
                    fd: number,
                    path_ptr: number,
                    path_len: number
                ) => {
                    // don't allow touching non-preopens
                    if (fd >= this.preopenCount) {
                        return WASI_ERRNO_INVAL;
                    }

                    const dir = this.fds[fd];
                    if (dir === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const preopenDirName = dir.getPreopenDirName();

                    const bytesToWrite = Math.min(
                        path_len,
                        preopenDirName.length
                    );

                    const encoder = new TextEncoder();
                    const dirBytes = encoder.encode(preopenDirName);
                    new Uint8Array(
                        this.memory.buffer,
                        path_ptr,
                        bytesToWrite
                    ).set(dirBytes.subarray(0, bytesToWrite));

                    return WASI_ERRNO_SUCCESS;
                },
                fd_fdstat_get: (fd: number, buf_ptr: number) => {
                    const memory = new DataView(this.memory.buffer);

                    if (fd === 0 || fd === 1 || fd === 2) {
                        memory.setUint8(buf_ptr, WASI_FILETYPE_REGULAR_FILE);
                    } else if (fd >= 3 && this.options.preopens !== undefined) {
                        memory.setUint8(buf_ptr, WASI_FILETYPE_DIRECTORY);
                    } else {
                        return WASI_ERRNO_BADF;
                    }

                    const fd_flags = 0;
                    memory.setUint16(buf_ptr + 2, fd_flags, true);

                    const rights_base = 0n;
                    memory.setBigUint64(buf_ptr + 8, rights_base, true);

                    const rights_inheriting = 0n;
                    memory.setBigUint64(buf_ptr + 16, rights_inheriting, true);

                    return WASI_ERRNO_SUCCESS;
                },
            },
        };
    }
}

class ProcExitError extends Error {
    exitCode: number;

    constructor(exitCode: number) {
        super();
        this.exitCode = exitCode;
    }
}
