// @ts-check

/**
 * @typedef {{
 *   sysCalls: WASISysCalls,
 *   version: "preview1",
 *   stdin?: number,
 *   stdout?: number,
 *   stderr?: number,
 *   args?: string[],
 *   env?: Record<string, string | undefined>
 *   preopens?: Record<string, string>,
 *   returnOnExit?: boolean,
 *   trace?: boolean,
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

const WASI_ERRNO_SUCCESS = 0;
const WASI_ERRNO_BADF = 8;
const WASI_ERRNO_NOENT = 2;
const WASI_ERRNO_INVAL = 28;

const WASI_FILETYPE_DIRECTORY = 3;
const WASI_FILETYPE_REGULAR_FILE = 4;

export class WASI {
    /** @param {WASIOptions} options */
    constructor(options) {
        /** @type {WASIOptions} */
        this.options = options;

        /** @type {string[]} */
        this.args = this.options.args ?? [];

        /** @type {Record<string, string>} */
        this.preopens = this.options.preopens ?? {};

        /** @type {Map<number, number>} */
        this.fileDescriptors = new Map();
        this.fileDescriptors.set(0, this.options.stdin ?? 0);
        this.fileDescriptors.set(1, this.options.stdout ?? 1);
        this.fileDescriptors.set(2, this.options.stderr ?? 2);

        /** @type {number} */
        this.nextFd = 3;

        /** @type {WebAssembly.Memory} */
        this.memory = /** @type {never} */ (void 0);

        /** @type {boolean} */
        this.returnOnExit = this.options.returnOnExit ?? true;

        /** @type {WASISysCalls} */
        this.ops = this.options.sysCalls;
    }

    /** @param {Omit<WASIOptions, 'sysCalls'>} options */
    static async NODE_FS(options) {
        const fs = await import("node:fs");
        const process = await import("node:process");

        /** @type {WASISysCalls} */
        const sysCalls = {
            processExit: (exitCode) => process.exit(exitCode),
            pathOpen: (path, mode) => fs.openSync(path, mode),
            fdRead: (fd, buffer) => fs.readSync(fd, buffer),
            fdWrite: (fd, buffer) => fs.writeSync(fd, buffer),
            fdClose: (fd) => fs.closeSync(fd),
        };

        return new WASI({ ...options, sysCalls });
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
            for (const [inFd, fd] of this.fileDescriptors) {
                if ([0, 1, 2].includes(inFd)) continue;

                this.ops.fdClose(fd);
            }
        }

        if (!this.returnOnExit) {
            this.ops.processExit(exitCode);
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
                    const dirNameIn = Object.keys(this.preopens)[dirfd - 3];
                    if (dirNameIn === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const dirNameReal = this.preopens[dirNameIn];
                    const pathBytes = new Uint8Array(
                        this.memory.buffer,
                        path_ptr,
                        path_len
                    );
                    const path = new TextDecoder()
                        .decode(pathBytes)
                        .replace(/\0/g, "");

                    const fullPath = `${dirNameReal}/${path}`;
                    try {
                        const fd = this.ops.pathOpen(fullPath, "r");
                        this.fileDescriptors.set(this.nextFd, fd);
                        const memory = new DataView(this.memory.buffer);
                        memory.setUint32(fd_ptr, this.nextFd, true);
                        this.nextFd += 1;
                        return WASI_ERRNO_SUCCESS;
                    } catch {
                        return WASI_ERRNO_NOENT;
                    }
                },
                /** @type {(fd: number, iovs_ptr: number, iovs_len: number, nread_ptr: number) => number} */
                fd_read: (fd, iovs_ptr, iovs_len, nread_ptr) => {
                    if (!this.fileDescriptors.has(fd)) {
                        return WASI_ERRNO_BADF;
                    }

                    const fdHandle = this.fileDescriptors.get(fd) ?? 0;
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

                        try {
                            const bytesRead = this.ops.fdRead(fdHandle, buffer);
                            totalBytesRead += bytesRead;

                            if (bytesRead < bufLen) {
                                break;
                            }
                        } catch {
                            return WASI_ERRNO_INVAL;
                        }
                    }

                    memory.setUint32(nread_ptr, totalBytesRead, true);
                    return WASI_ERRNO_SUCCESS;
                },
                /** @type {(fd: number, iovs_ptr: number, iovs_len: number, nwritten_ptr: number) => number} */
                fd_write: (fd, iovs_ptr, iovs_len, nwritten_ptr) => {
                    if (!this.fileDescriptors.has(fd)) {
                        return WASI_ERRNO_BADF;
                    }

                    const fdHandle = this.fileDescriptors.get(fd) ?? 0;
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

                        try {
                            const bytesWritten = this.ops.fdWrite(
                                fdHandle,
                                buffer
                            );
                            totalBytesWritten += bytesWritten;

                            if (bytesWritten < bufLen) {
                                break;
                            }
                        } catch {
                            return WASI_ERRNO_INVAL;
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
                    if (!this.fileDescriptors.has(fd)) {
                        return WASI_ERRNO_BADF;
                    }

                    const fdHandle = this.fileDescriptors.get(fd) ?? 0;
                    try {
                        this.ops.fdClose(fdHandle);
                        this.fileDescriptors.delete(fd);

                        return WASI_ERRNO_SUCCESS;
                    } catch {
                        return WASI_ERRNO_INVAL;
                    }
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
                    const memory = new DataView(this.memory.buffer);

                    const dirNameIn = Object.keys(this.preopens)[fd - 3];
                    if (dirNameIn === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    memory.setUint8(buf_ptr, 0); // dir

                    const dirPath = this.preopens[dirNameIn];
                    memory.setUint32(buf_ptr + 4, dirPath.length, true);

                    return WASI_ERRNO_SUCCESS;
                },
                /** @type {(fd: number, path_ptr: number, path_len: number) => number} */
                fd_prestat_dir_name: (fd, path_ptr, path_len) => {
                    const dirNameIn = Object.keys(this.preopens)[fd - 3];
                    if (dirNameIn === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const dirPath = this.preopens[dirNameIn];

                    const bytesToWrite = Math.min(path_len, dirPath.length);

                    const encoder = new TextEncoder();
                    const dirBytes = encoder.encode(dirPath);
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
