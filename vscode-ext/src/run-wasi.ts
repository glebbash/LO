import { Wasm, Readable } from "@vscode/wasm-wasi";

export type ProgramOptions = {
    processName: string;
    args: string[];
    cwdUri: import("vscode").Uri;
    module: WebAssembly.Module;
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

    const process = await wasm.createProcess(
        options.processName,
        options.module,
        {
            stdio: {
                in: { kind: "pipeIn", pipe: stdin },
                out: { kind: "pipeOut", pipe: stdout },
                err: { kind: "pipeOut", pipe: stderr },
            },
            args: options.args,
            mountPoints: [
                // TODO: figure out why this doesn't work
                // { kind: "workspaceFolder" },
                {
                    kind: "vscodeFileSystem",
                    uri: options.cwdUri,
                    mountPoint: "/",
                },
            ],
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
    let data = new Uint8Array();
    readable.onData((chunk) => {
        const newData = new Uint8Array(data.length + chunk.length);
        newData.set(data);
        newData.set(chunk, data.length);
        data = newData;
    });
    return Object.assign(readable, { get: () => data });
}
