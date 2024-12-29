// @ts-check

/**
 * @typedef {{
 *   version: "preview1",
 *   stdin?: number | FD,
 *   stdout?: number | FD,
 *   stderr?: number | FD,
 *   args?: string[],
 *   env?: Record<string, string | undefined>
 *   preopens?: Record<string, string>,
 *   returnOnExit?: boolean,
 *   trace?: boolean,
 *   sysCalls: WASISysCalls,
 * }} WASIOptions
 */

/**
 * @typedef {{
 *   processExit(exitCode: number): never,
 *   pathOpen(path: string, mode: 'r' | 'w'): number,
 *   fdRead(fd: number, buffer: Uint8Array): number,
 *   fdWrite(fd: number, buffer: Uint8Array): number,
 *   fdClose(fd: number): void,
 * }} WASISysCalls
 */

/**
 * @typedef {{
 *   getPreopenDirName(): string,
 *   openChild(fileName: string, mode: "r" | "w"): [number, FD | undefined],
 *   read(buffer: Uint8Array): [number, number | undefined],
 *   write(buffer: Uint8Array): [number, number | undefined],
 *   close(): [number],
 * }} FD
 */

const WASI_ERRNO_SUCCESS = 0;
const WASI_ERRNO_BADF = 8;
const WASI_ERRNO_NOENT = 2;
const WASI_ERRNO_INVAL = 28;

const WASI_FILETYPE_DIRECTORY = 3;
const WASI_FILETYPE_REGULAR_FILE = 4;

/** @implements {FD} */
class SysFD {
    /**
     * @param {number|null} sysFileFd
     * @param {string} sysDirName
     * @param {WASISysCalls} sysCalls
     */
    constructor(sysFileFd, sysDirName, sysCalls) {
        /** @type {number|null} */
        this.sysFileFd = sysFileFd;

        /** @type {string} */
        this.sysDirName = sysDirName;

        /** @type {WASISysCalls} */
        this.sysCalls = sysCalls;
    }

    getPreopenDirName() {
        return this.sysDirName;
    }

    /**
     * @param {string} fileName
     * @param {'r' | 'w'} mode
     * @returns {[number, FD|undefined]}
     */
    openChild(fileName, mode) {
        const fullPath = `${this.sysDirName}/${fileName}`;
        try {
            const sysFd = this.sysCalls.pathOpen(fullPath, mode);
            const fd = new SysFD(sysFd, fullPath, this.sysCalls);

            return [WASI_ERRNO_SUCCESS, fd];
        } catch {
            return [WASI_ERRNO_NOENT, undefined];
        }
    }

    /**
     * @param {Uint8Array} buffer
     * @returns {[number, number|undefined]}
     */
    read(buffer) {
        if (this.sysFileFd === null) {
            return [WASI_ERRNO_INVAL, undefined];
        }

        try {
            const bytesRead = this.sysCalls.fdRead(this.sysFileFd, buffer);
            return [WASI_ERRNO_SUCCESS, bytesRead];
        } catch {
            return [WASI_ERRNO_INVAL, undefined];
        }
    }

    /**
     * @param {Uint8Array} buffer
     * @returns {[number, number|undefined]}
     */
    write(buffer) {
        if (this.sysFileFd === null) {
            return [WASI_ERRNO_INVAL, undefined];
        }

        try {
            const bytesRead = this.sysCalls.fdWrite(this.sysFileFd, buffer);
            return [WASI_ERRNO_SUCCESS, bytesRead];
        } catch {
            return [WASI_ERRNO_INVAL, undefined];
        }
    }

    /** @returns {[number]} */
    close() {
        if (this.sysFileFd !== null && this.sysFileFd >= 3) {
            try {
                this.sysCalls.fdClose(this.sysFileFd);
            } catch {
                return [WASI_ERRNO_INVAL];
            }
        }
        return [WASI_ERRNO_SUCCESS];
    }
}

/** @implements {FD} */
class VirtualFD {
    constructor() {
        /** @type {Uint8Array} */
        this.contents = new Uint8Array();

        /** @type {number} */
        this.readCursor = 0;

        /** @type {Uint8Array[]} */
        this.writes = [];
    }

    /**
     * @param {Uint8Array} buffer
     * @returns {[number, number|undefined]}
     */
    read(buffer) {
        if (this.readCursor >= this.contents.length - 1) {
            return [WASI_ERRNO_SUCCESS, 0];
        }

        const bytesRead = Math.min(
            buffer.length,
            this.contents.length - this.readCursor
        );
        buffer.set(
            this.contents.subarray(this.readCursor, this.readCursor + bytesRead)
        );

        this.readCursor += bytesRead;

        return [WASI_ERRNO_SUCCESS, bytesRead];
    }

    /**
     * @param {Uint8Array} buffer
     * @returns {[number, number|undefined]}
     */
    write(buffer) {
        this.writes.push(buffer.slice());
        return [0, buffer.length];
    }

    /** @param {string} str */
    writeString(str) {
        this.write(new TextEncoder().encode(str));
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

    /** @returns {[number]} */
    close() {
        return [WASI_ERRNO_SUCCESS];
    }

    /** @returns {never} */
    getPreopenDirName() {
        throw new Error("Not supported");
    }

    /** @returns {never} */
    openChild() {
        throw new Error("Not supported");
    }
}

export class WASI {
    static FD = SysFD;
    static VirtualFD = VirtualFD;

    /** @param {WASIOptions} options */
    constructor(options) {
        /** @type {WASIOptions} */
        this.options = options;

        /** @type {string[]} */
        this.args = this.options.args ?? [];

        /** @type {number} */
        this.nextFd = 3;

        /** @type {WebAssembly.Memory} */
        this.memory = /** @type {never} */ (void 0);

        /** @type {boolean} */
        this.returnOnExit = this.options.returnOnExit ?? true;

        /** @type {WASISysCalls} */
        this.sysCalls = this.options.sysCalls;

        /** @type {FD[]} */
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

        /** @type {number} */
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

    /** @returns {WASISysCalls} */
    static DenoSysCalls() {
        /** @type {Map<number, Deno.FsFile>} */
        const files = new Map();

        const getFile = (/** @type {number} */ fd) => {
            if (fd === 0) {
                return /** @type {never} */ (Deno.stdin);
            } else if (fd === 1) {
                return /** @type {never} */ (Deno.stdout);
            } else if (fd === 2) {
                return /** @type {never} */ (Deno.stderr);
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

    /**
     * @param {WebAssembly.Instance} instance
     * @returns {number}
     */
    start(instance) {
        this.memory = /** @type {WebAssembly.Memory} */ (
            instance.exports.memory
        );

        let exitCode = 0;

        try {
            // @ts-expect-error: _start is a function
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
            imports.wasi_snapshot_preview1
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
                /** @type {(dirfd: number, dirflags: number, path_ptr: number, path_len: number, oflags: number, fs_rights_base: number, fs_rights_inheriting: number, fdflags: number, fd_ptr: number) => number} */
                path_open: (
                    dirfd,
                    _dirflags,
                    path_ptr,
                    path_len,
                    _oflags,
                    _fs_rights_base,
                    _fs_rights_inheriting,
                    _fdflags,
                    fd_ptr
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

                    const [err, childFile] = dir.openChild(path, "r");
                    if (childFile !== undefined) {
                        this.fds.push(childFile);

                        const childFd = this.fds.length - 1;
                        const memory = new DataView(this.memory.buffer);
                        memory.setUint32(fd_ptr, childFd, true);
                    }

                    return err;
                },
                /** @type {(fd: number, iovs_ptr: number, iovs_len: number, nread_ptr: number) => number} */
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
                            bufLen
                        );

                        const [err, bytesRead] = file.read(buffer);
                        if (bytesRead === undefined) {
                            return err;
                        }
                        totalBytesRead += bytesRead;

                        if (bytesRead < bufLen) {
                            break;
                        }
                    }

                    memory.setUint32(nread_ptr, totalBytesRead, true);
                    return WASI_ERRNO_SUCCESS;
                },
                /** @type {(fd: number, iovs_ptr: number, iovs_len: number, nwritten_ptr: number) => number} */
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
                            bufLen
                        );

                        const [err, bytesWritten] = file.write(buffer);
                        if (bytesWritten === undefined) {
                            return err;
                        }
                        totalBytesWritten += bytesWritten;

                        if (bytesWritten < bufLen) {
                            break;
                        }
                    }

                    memory.setUint32(nwritten_ptr, totalBytesWritten, true);
                    return WASI_ERRNO_SUCCESS;
                },
                /** @type {(fd: number, offset: number, whence: number, newoffset: number) => number} */
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

                    const [err] = file.close();

                    if (err === WASI_ERRNO_SUCCESS) {
                        this.fds.splice(fd, 1);
                    }

                    return err;
                },
                /** @type {(argc_ptr: number, argv_buf_size_ptr: number) => number} */
                args_sizes_get: (argc_ptr, argv_buf_size_ptr) => {
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
                /** @type {(fd: number, path_ptr: number, path_len: number) => number} */
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
            },
        };
    }
}

class ProcExitError extends Error {
    /** @param {number} exitCode */
    constructor(exitCode) {
        super();

        /** @type {number} */
        this.exitCode = exitCode;
    }
}
