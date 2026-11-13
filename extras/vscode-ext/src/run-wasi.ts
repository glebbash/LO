import { Wasm, Readable, MountPointDescriptor } from "@vscode/wasm-wasi/v1";

export type ProgramOptions = {
    processName: string;
    args: string[];
    cwdUri: import("vscode").Uri;
    module: WebAssembly.Module;
    memoryFs?: Record<string, Uint8Array>;
};

export type ProgramResult = {
    exitCode: number;
    stdout: Uint8Array;
    stderr: Uint8Array;
};

export async function runWasiProgram(
    options: ProgramOptions
): Promise<ProgramResult> {
    const { error } = checkValidWasiModule(options.module);
    if (error) {
        return {
            exitCode: -1,
            stdout: new Uint8Array(),
            stderr: new TextEncoder().encode(error),
        };
    }

    const wasm = await Wasm.load();
    const stdin = wasm.createWritable();
    const stdout = accumulateBytes(wasm.createReadable());
    const stderr = accumulateBytes(wasm.createReadable());

    let mountPoint: MountPointDescriptor = {
        kind: "vscodeFileSystem",
        uri: options.cwdUri,
        mountPoint: "/",
    };

    if (options.memoryFs) {
        const fileSystem = await wasm.createMemoryFileSystem();
        for (const [fileName, fileContent] of Object.entries(
            options.memoryFs
        )) {
            fileSystem.createFile(fileName, fileContent);
        }

        mountPoint = {
            kind: "memoryFileSystem",
            fileSystem,
            mountPoint: "/",
        };
    }

    const process = await wasm.createProcess(
        options.processName,
        options.module,
        {
            args: options.args,
            mountPoints: [mountPoint],
            stdio: {
                in: { kind: "pipeIn", pipe: stdin },
                out: { kind: "pipeOut", pipe: stdout },
                err: { kind: "pipeOut", pipe: stderr },
            },
            trace: true,
        }
    );

    const exitCode = await process.run();

    return {
        exitCode,
        stdout: stdout.get(),
        stderr: stderr.get(),
    };
}

function checkValidWasiModule(module: WebAssembly.Module) {
    const exports = WebAssembly.Module.exports(module);
    const startFn = exports.find(
        (e) => e.name === "_start" && e.kind === "function"
    );
    if (startFn === undefined) {
        return { error: "function `_start` is not exported" };
    }

    const memory = exports.find(
        (e) => e.name === "memory" && e.kind === "memory"
    );
    if (memory === undefined) {
        return { error: "memory `memory` is not exported" };
    }

    return {};
}

export function accumulateBytes(
    readable: Readable
): Readable & { get: () => Uint8Array } {
    let totalLength = 0;
    const chunks: Uint8Array[] = [];

    readable.onData((chunk: Uint8Array) => {
        chunks.push(chunk);
        totalLength += chunk.length;
    });

    return Object.assign(readable, {
        get: () => {
            const result = new Uint8Array(totalLength);
            let offset = 0;

            for (const chunk of chunks) {
                result.set(chunk, offset);
                offset += chunk.length;
            }

            return result;
        },
    });
}
