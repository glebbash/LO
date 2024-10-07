// @ts-check

/** @typedef {{
 *   version: "preview1",
 *   stdin?: number,
 *   stdout?: number,
 *   stderr?: number,
 *   args?: string[],
 *   env?: Record<string, string | undefined>
 *   preopens?: Record<string, string>,
 *   returnOnExit?: boolean,
 *   trace?: boolean,
 * }} WASIOptions */

import fs from "node:fs";
import process from "node:process";

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

                fs.closeSync(fd);
            }
        }

        if (!this.returnOnExit) {
            process.exit(exitCode);
        }

        return exitCode;
    }

    /** @returns {WebAssembly.Imports} */
    getImportObject() {
        const imports = this.getImportObjectRaw();
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
    getImportObjectRaw() {
        return {
            wasi_snapshot_preview1: {
                /**
                 * @param {number} dirfd
                 * @param {number} dirflags
                 * @param {number} path_ptr
                 * @param {number} path_len
                 * @param {number} oflags
                 * @param {number} fs_rights_base
                 * @param {number} fs_rights_inheriting
                 * @param {number} fdflags
                 * @param {number} fd_ptr
                 */
                path_open: (
                    dirfd,
                    dirflags,
                    path_ptr,
                    path_len,
                    oflags,
                    fs_rights_base,
                    fs_rights_inheriting,
                    fdflags,
                    fd_ptr
                ) => {
                    const _ignored = [
                        dirflags,
                        oflags,
                        fs_rights_base,
                        fs_rights_inheriting,
                        fdflags,
                    ];

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
                        const fd = fs.openSync(fullPath, "r");
                        this.fileDescriptors.set(this.nextFd, fd);
                        const memory = new DataView(this.memory.buffer);
                        memory.setUint32(fd_ptr, this.nextFd, true);
                        this.nextFd += 1;
                        return WASI_ERRNO_SUCCESS;
                    } catch {
                        return WASI_ERRNO_NOENT;
                    }
                },
                /**
                 * @param {number} fd
                 * @param {number} iovs_ptr
                 * @param {number} iovs_len
                 * @param {number} nread_ptr
                 */
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
                            const bytesRead = fs.readSync(fdHandle, buffer);
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
                /**
                 * @param {number} fd
                 * @param {number} iovs_ptr
                 * @param {number} iovs_len
                 * @param {number} nwritten_ptr
                 */
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
                            const bytesWritten = fs.writeSync(fdHandle, buffer);
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
                /**
                 * @param {number} fd
                 * @param {number} offset
                 * @param {number} whence
                 * @param {number} newoffset
                 */
                fd_seek: (fd, offset, whence, newoffset) => {
                    const _ = [fd, offset, whence, newoffset];
                    throw new Error("Not Implemented");
                },
                /** @param {number} fd */
                fd_close: (fd) => {
                    if (!this.fileDescriptors.has(fd)) {
                        return WASI_ERRNO_BADF;
                    }

                    const fdHandle = this.fileDescriptors.get(fd) ?? 0;
                    try {
                        fs.closeSync(fdHandle);
                        this.fileDescriptors.delete(fd);

                        return WASI_ERRNO_SUCCESS;
                    } catch {
                        return WASI_ERRNO_INVAL;
                    }
                },
                /**
                 * @param {number} argc_ptr
                 * @param {number} argv_buf_size_ptr
                 */
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
                /**
                 * @param {number} argv_ptr
                 * @param {number} argv_buf_ptr
                 */
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
                /** @param {number} exit_code */
                proc_exit: (exit_code) => {
                    throw new ProcExitError(exit_code);
                },
                /**
                 * @param {number} fd
                 * @param {number} buf
                 */
                fd_prestat_get: (fd, buf) => {
                    const memory = new DataView(this.memory.buffer);

                    const dirNameIn = Object.keys(this.preopens)[fd - 3];
                    if (dirNameIn === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    memory.setUint8(buf, 0); // dir

                    const dirPath = this.preopens[dirNameIn];
                    memory.setUint32(buf + 4, dirPath.length, true);

                    return WASI_ERRNO_SUCCESS;
                },
                /**
                 * @param {number} fd
                 * @param {number} path
                 * @param {number} path_len
                 */
                fd_prestat_dir_name: (fd, path, path_len) => {
                    const dirNameIn = Object.keys(this.preopens)[fd - 3];
                    if (dirNameIn === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const dirPath = this.preopens[dirNameIn];

                    const bytesToWrite = Math.min(path_len, dirPath.length);

                    const encoder = new TextEncoder();
                    const dirBytes = encoder.encode(dirPath);
                    new Uint8Array(this.memory.buffer, path, bytesToWrite).set(
                        dirBytes.subarray(0, bytesToWrite)
                    );

                    return WASI_ERRNO_SUCCESS;
                },
                /**
                 * @param {number} fd
                 * @param {number} buf
                 */
                fd_fdstat_get: (fd, buf) => {
                    const memory = new DataView(this.memory.buffer);

                    if (fd === 0 || fd === 1 || fd === 2) {
                        memory.setUint8(buf, WASI_FILETYPE_REGULAR_FILE);
                    } else if (fd >= 3 && this.options.preopens !== undefined) {
                        memory.setUint8(buf, WASI_FILETYPE_DIRECTORY);
                    } else {
                        return WASI_ERRNO_BADF;
                    }

                    const fd_flags = 0;
                    memory.setUint16(buf + 2, fd_flags, true);

                    const rights_base = 0n;
                    memory.setBigUint64(buf + 8, rights_base, true);

                    const rights_inheriting = 0n;
                    memory.setBigUint64(buf + 16, rights_inheriting, true);

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
