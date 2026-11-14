#!/usr/bin/env -S node --no-warnings

// Minimal embedder for lo.wasm built on node's own WASI implementation.
//
// NOTE: don't `import process from "node:process"` or use `process.stdout` here
//   as it breaks node's WASI impl cutting off >64kb compiler outputs when piped

// @ts-check

import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";
import { WASI } from "node:wasi";

const COMPILER_PATH = path.join(import.meta.dirname, "lo.wasm");
const INITIAL_COMPILER_PATH = "extras/initial-compiler/lo.wasm";
const COMPILER_DOWNLOAD_URL =
    "https://github.com/glebbash/LO/releases/latest/download/lo.wasm";

const USAGE = `Usage (embedder):
  ./lo.js build (b)                    // rebuilds lo.wasm (can be combined with other commands, ie: \`./lo js b test\`)
  ./lo.js test [--fast] [--skip-fmt]   // runs from \`test/_runners\` if present, \`--fast\` skips compiler comparison tests
  ./lo.js run <compiler-args> [-- ...] // compiles and runs a program
  ./lo.js runWasi <file.wasm> [args]   // runs any WASI module
  ./lo.js debugWasi <file.wasm>        // serves a WASI module on port 6969 for browser Dev Tools
  ./lo.js <any-other-command>          // passed through to lo.wasm (listed below)

-----------------------------------------------------------------------------

`;

await main(process.argv.slice(2));

async function main(/** @type {string[]} */ argv) {
    process.chdir(import.meta.dirname);

    if (!fs.existsSync(COMPILER_PATH)) {
        await askAndDownloadCompiler();
    }

    if (argv.length === 0) {
        // not process.stdout, see the note above
        fs.writeSync(1, USAGE);
    }

    let [command, ...rest] = argv;

    if (command === "build" || command === "b") {
        const output = await runWASIAndGetStdout(COMPILER_PATH, [
            "lo",
            "compile",
            "src/_cli.lo",
        ]);
        if (output === null) {
            return;
        }

        fs.writeFileSync(COMPILER_PATH, output);

        if (rest.length === 0) {
            return;
        }

        [command, ...rest] = rest; // continue with the next command
    }

    if (command === "run") {
        const sep = rest.indexOf("--");
        const compilerArgs = sep === -1 ? rest : rest.slice(0, sep);
        const programArgs = sep === -1 ? [] : rest.slice(sep + 1);

        const program = await runWASIAndGetStdout(COMPILER_PATH, [
            "lo",
            "compile",
            ...compilerArgs,
        ]);
        if (program === null) {
            console.error("Compilation failed, see compiler error(s) above");
            return;
        }

        const programPath = path.join(
            fs.mkdtempSync(path.join(os.tmpdir(), "lo-")),
            "main.wasm",
        );
        try {
            fs.writeFileSync(programPath, program);
            await runWasi(programPath, ["main.lo", ...programArgs]);
        } finally {
            fs.rmSync(path.dirname(programPath), { recursive: true });
        }
        return;
    }

    if (command === "test") {
        const samples = await importTestRunner("test-samples.js");
        await samples.runSampleTests(COMPILER_PATH, {
            skipFmt: rest.includes("--skip-fmt"),
        });

        if (!rest.includes("--fast") && fs.existsSync(INITIAL_COMPILER_PATH)) {
            const compare = await importTestRunner("compare-compilers.js");
            await compare.compareCompilers(
                COMPILER_PATH,
                INITIAL_COMPILER_PATH,
            );
        }
        return;

        async function importTestRunner(/** @type {string} */ fileName) {
            const runnerPath = path.join(
                import.meta.dirname,
                "test/_runners",
                fileName,
            );
            if (!fs.existsSync(runnerPath)) {
                console.error(
                    `${path.relative(".", runnerPath)} not found, the tests are not unpacked.\n` +
                        "Run `./lo.js eject` to unpack sources and tests first.",
                );
                process.exit(1);
            }
            return import(pathToFileURL(runnerPath).href);
        }
    }

    if (command === "runWasi") {
        await runWasi(rest[0], rest);
        return;
    }

    if (command === "debugWasi") {
        await debugWasi(rest[0]);
        return;
    }

    await runWasi(COMPILER_PATH, ["lo", ...argv]);
}

async function runWasi(
    /** @type {string} */ wasmPath,
    /** @type {string[]} */ args,
    stdout = 1,
) {
    const wasi = new WASI({
        version: "preview1",
        args,
        env: process.env,
        preopens: { ".": "." },
        stdin: 0,
        stdout,
        stderr: 2,
        returnOnExit: true,
    });

    const module = await WebAssembly.compile(fs.readFileSync(wasmPath));
    const instance = await WebAssembly.instantiate(module, {
        ...wasi.getImportObject(),
        console: { ...console },
    });

    try {
        const exitCode = wasi.start(instance);
        if (exitCode !== 0) {
            process.exitCode = exitCode;
        }

        return exitCode;
    } catch (err) {
        if (
            err instanceof WebAssembly.RuntimeError &&
            err.message.includes("unreachable")
        ) {
            const memory = /** @type {WebAssembly.Memory} */ (
                instance.exports.memory
            );
            const [indicator, code] = new Uint32Array(memory.buffer, 0, 2);
            if (indicator === 69420) {
                err.message = "Abort code " + code;
            }
        }
        throw err;
    }
}

async function debugWasi(/** @type {string} */ wasmPath) {
    const http = await import("node:http");

    http.createServer((req, res) => {
        if (req.method === "GET" && req.url === "/") {
            res.setHeader("Content-Type", "text/html");
            res.end(`
                <script type="module">
                    import { init, WASI } from "https://esm.sh/@wasmer/wasi@1.2.2";

                    const module = await WebAssembly.compile(
                        await fetch('./index.wasm').then((r) => r.arrayBuffer())
                    );

                    await init();
                    const wasi = new WASI({});
                    await wasi.instantiate(module, {});
                    const exitCode = wasi.start();
                    console.log({
                        exitCode,
                        out: wasi.getStdoutString(),
                        err: wasi.getStderrString(),
                    });
                </script>
            `);
            return;
        }

        if (req.method === "GET" && req.url === "/index.wasm") {
            res.setHeader("Content-Type", "application/wasm");
            res.end(fs.readFileSync(wasmPath));
            return;
        }

        res.statusCode = 404;
        res.end("Not Found");
    }).listen(6969, () => {
        console.log("Debug server running at http://localhost:6969/");
    });
}

async function runWASIAndGetStdout(
    /** @type {string} */ wasmPath,
    /** @type {string[]} */ args,
) {
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), "lo-"));
    const outPath = path.join(dir, "stdout");
    const outFd = fs.openSync(outPath, "w+");
    try {
        const exitCode = await runWasi(wasmPath, args, outFd);
        return exitCode === 0 ? fs.readFileSync(outPath) : null;
    } finally {
        fs.closeSync(outFd);
        fs.rmSync(dir, { recursive: true });
    }
}

async function askAndDownloadCompiler() {
    process.stderr.write(
        `lo.wasm not found. Press Enter to download it from ${COMPILER_DOWNLOAD_URL} (any other key to exit) `,
    );

    const stdin = process.stdin;
    if (stdin.isTTY) {
        stdin.setRawMode(true);
    }

    const key = await new Promise((resolve) => {
        stdin.once("data", resolve);
        stdin.once("end", () => resolve(null));
    });
    if (stdin.isTTY) {
        stdin.setRawMode(false);
    }

    stdin.pause();
    process.stderr.write("\n");

    const isEnter = key !== null && (key[0] === 0x0d || key[0] === 0x0a);
    if (!isEnter) {
        console.error("lo.wasm is required, exiting.");
        process.exit(1);
    }

    const res = await fetch(COMPILER_DOWNLOAD_URL);
    if (!res.ok) {
        console.error(`Download failed: ${res.status} ${res.statusText}`);
        process.exit(1);
    }

    fs.writeFileSync(COMPILER_PATH, Buffer.from(await res.arrayBuffer()));
    console.error("Downloaded lo.wasm");
}
