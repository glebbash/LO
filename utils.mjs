#!/usr/bin/env -S node --no-warnings
// @ts-check

import { WASI } from "node:wasi";
import process from "node:process";
import { randomUUID } from "node:crypto";
import { open, readFile, unlink } from "node:fs/promises";

const COMPILER_PATH = "./target/wasm32-unknown-unknown/release/lo.wasm";

const COMMANDS = {
    compile: compileCommand,
    run: runCommand,
    runWasi: runWasiCommand,
};

main();

function main() {
    const command = process.argv[2];
    const args = process.argv.slice(3);

    if (!(command in COMMANDS)) {
        console.log("Invalid command:", command);
        process.exit(1);
    }

    COMMANDS[command](args);
}

async function compileCommand() {
    return runWASI(await readFile(COMPILER_PATH), {
        preopens: { ".": "examples" },
        returnOnExit: false,
    });
}

async function runCommand() {
    let compilerArgs = process.argv.slice(3);
    let programArgs = [];

    let programArgsSeparator = compilerArgs.indexOf("--");
    if (programArgsSeparator !== -1) {
        compilerArgs = compilerArgs.slice(0, programArgsSeparator);
        programArgs = compilerArgs.slice(programArgsSeparator + 1);
    }

    const program = await runWithTmpFile(async (stdout, stdoutFile) => {
        const exitCode = /** @type {unknown} */ (
            await runWASI(await readFile(COMPILER_PATH), {
                stdout: stdout.fd,
                preopens: { ".": "examples" },
                args: ["compiler.wasm", ...compilerArgs],
            })
        );

        if (exitCode !== 0) {
            throw new Error("Compilation failed, see compiler error above");
        }

        return readFile(stdoutFile);
    }).catch((err) => {
        console.error(err.message);
        process.exit(1);
    });

    await runWASI(program, {
        preopens: { ".": "examples" },
        returnOnExit: false,
        args: ["main.lo", ...programArgs],
    });
}

/** @param {string[]} args */
async function runWasiCommand(args) {
    const filePath = new URL(args[0], import.meta.url);
    const input = await readFile(filePath);
    await runWASI(input, {
        args: args.slice(1),
        env: process.env,
    });
}

// utils

/**
 * @param {BufferSource} data
 * @param {Omit<import("node:wasi").WASIOptions, "version">} [wasiOptions]
 */
async function runWASI(data, wasiOptions) {
    const wasi = new WASI({ version: "preview1", ...wasiOptions });

    const wasm = await WebAssembly.compile(data);
    const instance = await WebAssembly.instantiate(
        wasm,
        // @ts-ignore
        wasi.getImportObject()
    );

    return wasi.start(instance);
}

/**
 * @template T
 * @param {(file: import("node:fs/promises").FileHandle, fileName: string) => T} run
 */
async function runWithTmpFile(run) {
    const mockOutputFileName = `${randomUUID()}.tmp`;
    const mockOutputFile = await open(mockOutputFileName, "w+");

    try {
        return await run(mockOutputFile, mockOutputFileName);
    } finally {
        await mockOutputFile.close();
        await unlink(mockOutputFileName);
    }
}
