#!/usr/bin/env -S node --no-warnings
// @ts-check

import { WASI } from "node:wasi";
import process from "node:process";
import { randomUUID } from "node:crypto";
import { open, readFile, unlink } from "node:fs/promises";

const COMPILER_PATH = "./target/wasm32-unknown-unknown/release/lole_lisp.wasm";

const COMMANDS = {
    compile: compileCommand,
    run: runCommand,
    minify: minifyCommand,
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
    const program = await runWithTmpFile(async (stdout, stdoutFile) => {
        await runWASI(await readFile(COMPILER_PATH), {
            stdout: stdout.fd,
            preopens: { ".": "examples" },
            returnOnExit: false,
        });

        return readFile(stdoutFile);
    });

    await runWASI(program, {
        preopens: { ".": "examples" },
        returnOnExit: false,
    });
}

/** @param {string[]} args */
async function minifyCommand(args) {
    const filePath = new URL(args[0], import.meta.url);
    const input = await readFile(filePath, "utf-8");
    const minified = doMinify(input);
    console.log(minified);

    /** @param {string} input */
    function doMinify(input) {
        while (true) {
            const trimmed = input
                .replace(/;.*\n/g, "")
                .replace(/\s+/g, " ")
                .replace(/ ([\)\}\]])/g, "$1")
                .replace(/([\(\{\[]) ([\(\{\[])/g, "$1$2")
                .replace(/([\)\}\]]) ([\(\{\[])/g, "$1$2");

            if (trimmed.length === input.length) break;
            input = trimmed;
        }

        return input
            .replace(/\(export /g, "\n(export ")
            .replace(/\(fn /g, "\n(fn ")
            .replace(/\(struct /g, "\n(struct ")
            .replace(/\(enum /g, "\n(enum ")
            .replace(/\(global /g, "\n(global ");
    }
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
 * @param {import("node:wasi").WASIOptions} [wasiOptions]
 */
async function runWASI(data, wasiOptions) {
    // @ts-ignore
    const wasi = new WASI({ version: "preview1", ...wasiOptions });

    const wasm = await WebAssembly.compile(data);
    const instance = await WebAssembly.instantiate(
        wasm,
        // @ts-ignore
        wasi.getImportObject()
    );

    wasi.start(instance);
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
