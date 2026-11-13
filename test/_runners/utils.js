// @ts-check

// Helpers for the test runners: loading and running compiler and program binaries,
//   and a custom WASI preview1 implementation with support for virtual (in-memory)
//   file descriptors, used to feed stdin and capture stdout/stderr.

export async function loadLoCompiler(
    /** @type {Uint8Array} */ compilerWasmBinary,
) {
    const mod = await WebAssembly.compile(
        /** @type {Uint8Array<ArrayBuffer>} */ (compilerWasmBinary),
    );

    return async (
        args = ["help"],
        {
            stdin = new WASI.VirtualFD(),
            stdout = new WASI.VirtualFD(),
            stderr = new WASI.VirtualFD(),
        } = {},
    ) => {
        const wasi = new WASI({
            version: "preview1",
            stdin,
            stdout,
            stderr,
            args: ["lo", ...args],
            preopens: { ".": "." },
            sysCalls: await WASI.NodeSysCalls(),
        });

        const instance = await WebAssembly.instantiate(mod, {
            ...wasi.getImportObject(),
            ...{ console: { ...console } },
        });

        try {
            const exitCode = wasi.start(instance);

            if (exitCode ?? 0 !== 0) {
                throw new Error(stderr.flushAndReadUtf8());
            }

            return stdout.flushAndRead();
        } catch (err) {
            const errorMessage = stderr.flushAndReadUtf8();
            if (errorMessage !== "") {
                if (
                    err instanceof WebAssembly.RuntimeError &&
                    err.message.includes("unreachable")
                ) {
                    err.message = errorMessage;
                } else {
                    throw new Error(errorMessage);
                }
            }

            throw err;
        }
    };
}

export async function loadWasm(
    /** @type {Uint8Array<ArrayBuffer>} */ data,
    /** @type {WebAssembly.Imports | undefined} */ imports = undefined,
) {
    const mod = await WebAssembly.instantiate(data, imports);
    return /** @type {any} */ (mod.instance.exports);
}

export async function runWASI(
    /** @type {Uint8Array} */ data,
    /** @type {Omit<WASIOptions, "version" | "sysCalls"> | undefined} */ wasiOptions = undefined,
    /** @type {Record<string, unknown>} */ additionalImports = {},
) {
    const wasi = new WASI({
        version: "preview1",
        sysCalls: await WASI.NodeSysCalls(),
        ...wasiOptions,
    });

    const wasm = await WebAssembly.compile(
        /** @type {Uint8Array<ArrayBuffer>} */ (data),
    );
    const instance = await WebAssembly.instantiate(wasm, {
        ...wasi.getImportObject(),
        ...{ console: { ...console } },
        ...additionalImports,
    });

    try {
        return wasi.start(instance);
    } catch (err) {
        if (err instanceof WebAssembly.RuntimeError) {
            if (err.message.includes("unreachable")) {
                const memory = /** @type {WebAssembly.Memory} */ (
                    instance.exports.memory
                );
                const [errorIndicator, errorCode] = new Uint32Array(
                    memory.buffer.slice(0, 8),
                );

                if (errorIndicator === 69420) {
                    err.message = "Abort code " + errorCode;
                }
            }
        }
        throw err;
    }
}

// WASI shim

/**
 * @typedef {{
 *   version: "preview1",
 *   stdin?: number | FD,
 *   stdout?: number | FD,
 *   stderr?: number | FD,
 *   args?: string[],
 *   env?: Record<string, string | undefined>,
 *   preopens?: Record<string, string>,
 *   returnOnExit?: boolean,
 *   trace?: boolean,
 *   sysCalls: WASISysCalls,
 * }} WASIOptions
 */

/**
 * @typedef {{
 *   processExit(exitCode: number): never,
 *   pathOpen(path: string, mode: "r" | "w" | "wx"): number,
 *   fdRead(fd: number, buffer: Uint8Array): number,
 *   fdWrite(fd: number, buffer: Uint8Array): number,
 *   fdClose(fd: number): void,
 *   listDir(path: string): Array<{ name: string; isDirectory: boolean }>,
 *   mkdir(path: string): void,
 * }} WASISysCalls
 */

/**
 * @typedef {{
 *   getPreopenDirName(): string,
 *   openChild(
 *     fileName: string,
 *     mode: "r" | "w" | "wx",
 *   ): { code: number, fd?: FD },
 *   createChildDir(dirName: string): number,
 *   read(buffer: Uint8Array): { code: number, nread: number },
 *   write(buffer: Uint8Array): { code: number, nwritten: number },
 *   close(): number,
 *   listDir(): {
 *     code: number,
 *     entries?: Array<{ name: string, isDirectory: boolean }>,
 *   },
 * }} FD
 */

const WASI_ERRNO_SUCCESS = 0;
const WASI_ERRNO_BADF = 8;
const WASI_ERRNO_NOENT = 2;
const WASI_ERRNO_EXIST = 20;
const WASI_ERRNO_INVAL = 28;

const WASI_FILETYPE_DIRECTORY = 3;
const WASI_FILETYPE_REGULAR_FILE = 4;

/** @implements {FD} */
class SysFD {
    /** @type {number | null} */
    sysFileFd;
    /** @type {string} */
    sysDirName;
    /** @type {WASISysCalls} */
    sysCalls;

    constructor(
        /** @type {number | null} */ sysFileFd,
        /** @type {string} */ sysDirName,
        /** @type {WASISysCalls} */ sysCalls,
    ) {
        this.sysFileFd = sysFileFd;
        this.sysDirName = sysDirName;
        this.sysCalls = sysCalls;
    }

    getPreopenDirName() {
        return this.sysDirName;
    }

    openChild(
        /** @type {string} */ fileName,
        /** @type {"r" | "w" | "wx"} */ mode,
    ) {
        const fullPath = `${this.sysDirName}/${fileName}`;
        try {
            const sysFd = this.sysCalls.pathOpen(fullPath, mode);
            const fd = new SysFD(sysFd, fullPath, this.sysCalls);

            return { code: WASI_ERRNO_SUCCESS, fd };
        } catch (err) {
            if (/** @type {{ code?: string }} */ (err)?.code === "EEXIST") {
                return { code: WASI_ERRNO_EXIST, fd: undefined };
            }
            return { code: WASI_ERRNO_NOENT, fd: undefined };
        }
    }

    createChildDir(/** @type {string} */ dirName) {
        const fullPath = `${this.sysDirName}/${dirName}`;
        try {
            this.sysCalls.mkdir(fullPath);
            return WASI_ERRNO_SUCCESS;
        } catch (err) {
            if (/** @type {{ code?: string }} */ (err)?.code === "EEXIST") {
                return WASI_ERRNO_EXIST;
            }
            return WASI_ERRNO_NOENT;
        }
    }

    listDir() {
        try {
            return {
                code: WASI_ERRNO_SUCCESS,
                entries: this.sysCalls.listDir(this.sysDirName),
            };
        } catch {
            return { code: WASI_ERRNO_NOENT, entries: undefined };
        }
    }

    read(/** @type {Uint8Array} */ buffer) {
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

    write(/** @type {Uint8Array} */ buffer) {
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

/** @implements {FD} */
class VirtualFD {
    /** @type {Uint8Array<ArrayBuffer>} */
    contents;
    /** @type {number} */
    readCursor;
    /** @type {Uint8Array[]} */
    writes;

    constructor() {
        this.contents = new Uint8Array();
        this.readCursor = 0;
        this.writes = [];
    }

    read(/** @type {Uint8Array} */ buffer) {
        if (this.readCursor >= this.contents.length - 1) {
            return { code: WASI_ERRNO_SUCCESS, nread: 0 };
        }

        const bytesRead = Math.min(
            buffer.length,
            this.contents.length - this.readCursor,
        );
        buffer.set(
            this.contents.subarray(
                this.readCursor,
                this.readCursor + bytesRead,
            ),
        );

        this.readCursor += bytesRead;

        return { code: WASI_ERRNO_SUCCESS, nread: bytesRead };
    }

    write(/** @type {Uint8Array} */ buffer) {
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

    /** @returns {never} */
    getPreopenDirName() {
        throw new Error("Not supported");
    }

    /** @returns {never} */
    openChild() {
        throw new Error("Not supported");
    }

    /** @returns {never} */
    createChildDir() {
        throw new Error("Not supported");
    }

    /** @returns {never} */
    listDir() {
        throw new Error("Not supported");
    }
}

export class WASI {
    static FD = SysFD;
    static VirtualFD = VirtualFD;

    /** @type {WASIOptions} */
    options;
    /** @type {string[]} */
    args;
    /** @type {number} */
    nextFd;
    /** @type {WebAssembly.Memory} */
    memory;
    /** @type {boolean} */
    returnOnExit;
    /** @type {WASISysCalls} */
    sysCalls;
    /** @type {FD[]} */
    fds;
    /** @type {number} */
    preopenCount;

    constructor(/** @type {WASIOptions} */ options) {
        this.options = options;
        this.args = this.options.args ?? [];
        this.nextFd = 3;
        this.memory = /** @type {never} */ (void 0);
        this.returnOnExit = this.options.returnOnExit ?? true;
        this.sysCalls = this.options.sysCalls;
        this.fds = [];

        if (typeof this.options.stdin === "object") {
            this.fds.push(this.options.stdin);
        } else {
            this.fds.push(
                new SysFD(this.options.stdin ?? 0, "<stdin>", this.sysCalls),
            );
        }

        if (typeof this.options.stdout === "object") {
            this.fds.push(this.options.stdout);
        } else {
            this.fds.push(
                new SysFD(this.options.stdout ?? 1, "<stdout>", this.sysCalls),
            );
        }

        if (typeof this.options.stderr === "object") {
            this.fds.push(this.options.stderr);
        } else {
            this.fds.push(
                new SysFD(this.options.stderr ?? 2, "<stderr>", this.sysCalls),
            );
        }

        for (const path of Object.values(this.options.preopens ?? {})) {
            this.fds.push(new SysFD(null, path, this.sysCalls));
        }

        this.preopenCount = this.fds.length;
    }

    /** @returns {Promise<WASISysCalls>} */
    static async NodeSysCalls() {
        const fs = await import("node:fs");
        const process = await import("node:process");

        return {
            processExit: (exitCode) => process.exit(exitCode),
            pathOpen: (path, mode) => fs.openSync(path, mode),
            fdRead: (fd, buffer) => fs.readSync(fd, buffer),
            fdWrite: (fd, buffer) => fs.writeSync(fd, buffer),
            mkdir: (path) => fs.mkdirSync(path),
            fdClose: (fd) => {
                try {
                    fs.closeSync(fd);
                } catch (err) {
                    console.error(err);
                    process.exit(1);
                }
            },
            listDir: (path) => [
                { name: ".", isDirectory: true },
                { name: "..", isDirectory: true },
                ...fs
                    .readdirSync(path, { withFileTypes: true })
                    .map((entry) => ({
                        name: entry.name,
                        isDirectory: entry.isDirectory(),
                    }))
                    .sort((a, b) =>
                        a.name < b.name ? -1 : a.name > b.name ? 1 : 0,
                    ),
            ],
        };
    }

    /**
     * @returns {number}
     */
    start(/** @type {WebAssembly.Instance} */ instance) {
        this.memory = /** @type {WebAssembly.Memory} */ (
            instance.exports.memory
        );

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

    /** @returns {WebAssembly.Imports} */
    getImportObject() {
        const imports = this.#getImportObject();
        if (!this.options.trace) {
            return imports;
        }

        for (const [fnName, fn] of Object.entries(
            imports.wasi_snapshot_preview1,
        )) {
            imports.wasi_snapshot_preview1[fnName] = (
                /** @type {unknown[]} */ ...args
            ) => {
                console.log(`[WASI] ${fnName}(${args.join(", ")})`);

                // @ts-ignore: fn is a function
                return fn(...args);
            };
        }

        return imports;
    }

    /** @returns {WebAssembly.Imports} */
    #getImportObject() {
        return {
            wasi_snapshot_preview1: {
                /**
                 * @type {(
                 *   dirfd: number,
                 *   _dirflags: number,
                 *   path_ptr: number,
                 *   path_len: number,
                 *   oflags: number,
                 *   _fs_rights_base: number,
                 *   _fs_rights_inheriting: number,
                 *   _fdflags: number,
                 *   fd_ptr: number,
                 * ) => number}
                 */
                path_open: (
                    dirfd,
                    _dirflags,
                    path_ptr,
                    path_len,
                    oflags,
                    _fs_rights_base,
                    _fs_rights_inheriting,
                    _fdflags,
                    fd_ptr,
                ) => {
                    const dir = this.fds[dirfd];
                    if (dir === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const pathBytes = new Uint8Array(
                        this.memory.buffer,
                        path_ptr,
                        path_len,
                    );
                    const path = new TextDecoder()
                        .decode(pathBytes)
                        .replace(/\0/g, "");

                    const OFLAGS_CREAT = 1;
                    const OFLAGS_EXCL = 4;
                    const mode =
                        (oflags & OFLAGS_CREAT) !== 0 &&
                        (oflags & OFLAGS_EXCL) !== 0
                            ? "wx"
                            : "r";

                    const { code: err, fd: childFile } = dir.openChild(
                        path,
                        mode,
                    );
                    if (childFile !== undefined) {
                        this.fds.push(childFile);

                        const childFd = this.fds.length - 1;
                        const memory = new DataView(this.memory.buffer);
                        memory.setUint32(fd_ptr, childFd, true);
                    }

                    return err;
                },
                /**
                 * @type {(
                 *   dirfd: number,
                 *   path_ptr: number,
                 *   path_len: number,
                 * ) => number}
                 */
                path_create_directory: (dirfd, path_ptr, path_len) => {
                    const dir = this.fds[dirfd];
                    if (dir === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const pathBytes = new Uint8Array(
                        this.memory.buffer,
                        path_ptr,
                        path_len,
                    );
                    const path = new TextDecoder()
                        .decode(pathBytes)
                        .replace(/\0/g, "");

                    return dir.createChildDir(path);
                },
                /**
                 * @type {(
                 *   fd: number,
                 *   iovs_ptr: number,
                 *   iovs_len: number,
                 *   nread_ptr: number,
                 * ) => number}
                 */
                fd_read: (fd, iovs_ptr, iovs_len, nread_ptr) => {
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
                            bufLen,
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
                /**
                 * @type {(
                 *   fd: number,
                 *   iovs_ptr: number,
                 *   iovs_len: number,
                 *   nwritten_ptr: number,
                 * ) => number}
                 */
                fd_write: (fd, iovs_ptr, iovs_len, nwritten_ptr) => {
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
                            bufLen,
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
                /** @type {(fd: number) => number} */
                fd_close: (fd) => {
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
                /**
                 * @type {(
                 *   argc_ptr: number,
                 *   argv_buf_size_ptr: number,
                 * ) => number}
                 */
                args_sizes_get: (argc_ptr, argv_buf_size_ptr) => {
                    const memory = new DataView(this.memory.buffer);

                    const argc = this.args.length;
                    memory.setUint32(argc_ptr, argc, true);

                    const argvBufferSize = this.args.reduce(
                        (acc, arg) => acc + arg.length + 1,
                        0,
                    );
                    memory.setUint32(argv_buf_size_ptr, argvBufferSize, true);

                    return WASI_ERRNO_SUCCESS;
                },
                /** @type {(argv_ptr: number, argv_buf_ptr: number) => number} */
                args_get: (argv_ptr, argv_buf_ptr) => {
                    const memory = new DataView(this.memory.buffer);

                    const encoder = new TextEncoder();
                    let bufferOffset = argv_buf_ptr;

                    for (const [i, arg] of Object.entries(this.args)) {
                        const argBytes = encoder.encode(arg + "\0");
                        new Uint8Array(
                            this.memory.buffer,
                            bufferOffset,
                            argBytes.length,
                        ).set(argBytes);
                        memory.setUint32(
                            argv_ptr + Number(i) * 4,
                            bufferOffset,
                            true,
                        );
                        bufferOffset += argBytes.length;
                    }

                    return WASI_ERRNO_SUCCESS;
                },
                /** @type {(exit_code: number) => never} */
                proc_exit: (exit_code) => {
                    throw new ProcExitError(exit_code);
                },
                /** @type {(fd: number, buf_ptr: number) => number} */
                fd_prestat_get: (fd, buf_ptr) => {
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
                /**
                 * @type {(
                 *   fd: number,
                 *   path_ptr: number,
                 *   path_len: number,
                 * ) => number}
                 */
                fd_prestat_dir_name: (fd, path_ptr, path_len) => {
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
                        preopenDirName.length,
                    );

                    const encoder = new TextEncoder();
                    const dirBytes = encoder.encode(preopenDirName);
                    new Uint8Array(
                        this.memory.buffer,
                        path_ptr,
                        bytesToWrite,
                    ).set(dirBytes.subarray(0, bytesToWrite));

                    return WASI_ERRNO_SUCCESS;
                },
                /** @type {(fd: number, buf_ptr: number) => number} */
                fd_fdstat_get: (fd, buf_ptr) => {
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
                /**
                 * @type {(
                 *   clock_id: number,
                 *   _precision: number,
                 *   time: number,
                 * ) => number}
                 */
                clock_time_get: (clock_id, _precision, time) => {
                    if (clock_id !== 1) {
                        return WASI_ERRNO_INVAL;
                    }

                    const now = BigInt(
                        Math.floor(performance.now() * 1_000_000),
                    );
                    const memory = new DataView(this.memory.buffer);
                    memory.setBigUint64(time, now, true);
                    return WASI_ERRNO_SUCCESS;
                },
                /**
                 * @type {(
                 *   fd: number,
                 *   buf_ptr: number,
                 *   buf_len: number,
                 *   cookie: bigint,
                 *   bufused_ptr: number,
                 * ) => number}
                 */
                fd_readdir: (fd, buf_ptr, buf_len, cookie, bufused_ptr) => {
                    const WASI_DIRENT_SIZE = 24; // d_next: u64, d_ino: u64, d_namlen: u32, d_type: u8 (+ padding)

                    const dir = this.fds[fd];
                    if (dir === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const { code, entries } = dir.listDir();
                    if (code !== WASI_ERRNO_SUCCESS || entries === undefined) {
                        return code;
                    }

                    // the directory listing is re-read on every call,
                    // a plain index into it is used as the dirent cookie
                    const startIndex = Number(cookie);

                    const memory = new DataView(this.memory.buffer);
                    const bufBytes = new Uint8Array(this.memory.buffer);
                    const encoder = new TextEncoder();

                    let bufused = 0;
                    for (let i = startIndex; i < entries.length; i++) {
                        const entry = entries[i];
                        const nameBytes = encoder.encode(entry.name);
                        const entrySize = WASI_DIRENT_SIZE + nameBytes.length;
                        const direntType = entry.isDirectory
                            ? WASI_FILETYPE_DIRECTORY
                            : WASI_FILETYPE_REGULAR_FILE;

                        const dirent = new Uint8Array(entrySize);
                        const direntView = new DataView(dirent.buffer);
                        direntView.setBigUint64(0, BigInt(i + 1), true); // d_next
                        direntView.setBigUint64(8, 0n, true); // d_ino
                        direntView.setUint32(16, nameBytes.length, true); // d_namlen
                        direntView.setUint8(20, direntType); // d_type
                        dirent.set(nameBytes, WASI_DIRENT_SIZE);

                        // per spec, a trailing entry that doesn't fully fit is truncated
                        const remaining = buf_len - bufused;
                        const bytesToCopy = Math.min(remaining, entrySize);
                        bufBytes.set(
                            dirent.subarray(0, bytesToCopy),
                            buf_ptr + bufused,
                        );
                        bufused += bytesToCopy;

                        if (bytesToCopy < entrySize) {
                            break;
                        }
                    }

                    memory.setUint32(bufused_ptr, bufused, true);
                    return WASI_ERRNO_SUCCESS;
                },
            },
        };
    }
}

class ProcExitError extends Error {
    /** @type {number} */
    exitCode;

    constructor(/** @type {number} */ exitCode) {
        super();
        this.exitCode = exitCode;
    }
}
