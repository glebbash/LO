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

export async function runProgram(
    options: ProgramOptions & { useNodeWasi: boolean }
): Promise<ProgramResult> {
    return options.useNodeWasi
        ? runProgramNode(options)
        : runProgramCode(options);
}

async function runProgramNode(options: ProgramOptions): Promise<ProgramResult> {
    const { WASI } = await import("node:wasi");
    const fs = await import("node:fs/promises");
    const crypto = await import("node:crypto");

    const tmpDir = options.cwdUri.fsPath + "/tmp";
    const stdoutFile = `${tmpDir}/${crypto.randomUUID()}.tmp`;
    const stderrFile = `${tmpDir}/${crypto.randomUUID()}.tmp`;
    let stdoutHandle: import("node:fs/promises").FileHandle | undefined;
    let stderrHandle: import("node:fs/promises").FileHandle | undefined;

    try {
        stdoutHandle = await fs.open(stdoutFile, "w+");
        stderrHandle = await fs.open(stderrFile, "w+");

        const wasi = new WASI({
            stdout: stdoutHandle.fd,
            stderr: stderrHandle.fd,
            args: [options.processName, ...options.args],
            preopens: { ".": options.cwdUri.fsPath },
            returnOnExit: true,
        });

        const instance = await WebAssembly.instantiate(options.module, {
            wasi_snapshot_preview1: wasi.wasiImport,
        });
        const exitCode = wasi.start(instance);
        return {
            exitCode,
            stdout: await fs.readFile(stdoutFile),
            stderr: await fs.readFile(stderrFile),
        };
    } finally {
        await stdoutHandle?.close();
        await fs.unlink(stdoutFile);

        await stderrHandle?.close();
        await fs.unlink(stderrFile);
    }
}

export async function runProgramCode(
    options: ProgramOptions
): Promise<ProgramResult> {
    const { Wasm } = await import("@vscode/wasm-wasi");

    const wasm = await Wasm.api();
    const stdin = wasm.createWritable();
    const [stdout, stdoutBytes] = accumulateBytes(wasm.createReadable());
    const [stderr, stderrBytes] = accumulateBytes(wasm.createReadable());

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
        stdout: stdoutBytes.get(),
        stderr: stderrBytes.get(),
    };
}

type Readable = import("@vscode/wasm-wasi").Readable;

function accumulateBytes(
    readable: Readable
): [Readable, { get: () => Uint8Array }] {
    let data = new Uint8Array();
    readable.onData((chunk) => {
        const newData = new Uint8Array(data.length + chunk.length);
        newData.set(data);
        newData.set(chunk, data.length);
        data = newData;
    });
    return [readable, { get: () => data }];
}
