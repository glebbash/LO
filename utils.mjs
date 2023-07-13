#!/usr/bin/env -S node --no-warnings
// @ts-check

import { readFile } from "node:fs/promises";
import { WASI } from "node:wasi";
import process from "node:process";

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

/** @param {string[]} args */
async function compileCommand(args) {
    const filePath = new URL(args[0], import.meta.url);
    const compiler = await loadWasm(await readFile(COMPILER_PATH));
    const program = await compile(compiler, await readFile(filePath));
    process.stdout.write(program);
}

/** @param {string[]} args */
async function runCommand(args) {
    const filePath = new URL(args[0], import.meta.url);
    const compiler = await loadWasm(await readFile(COMPILER_PATH));
    const program = await compile(compiler, await readFile(filePath));
    await runWASI(program, {
        args: args.slice(1),
        env: process.env,
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
 * @param {WebAssembly.Imports} [imports]
 * @returns {Promise<any>}
 */
async function loadWasm(data, imports) {
    const mod = await WebAssembly.instantiate(data, imports);
    return mod.instance.exports;
}

/**
 * @param {any} compiler
 * @param {Buffer} source
 */
async function compile(compiler, source) {
    const src = storeData(
        compiler.memory,
        compiler.mem_alloc(source.byteLength),
        source
    );

    const [ok, outPtr, outSize] = compiler.compile(src.ptr, src.size);

    const output = new Uint8Array(outSize);
    output.set(new Uint8Array(compiler.memory.buffer, outPtr, outSize));

    compiler.mem_free(src.ptr, src.size);
    compiler.mem_free(outPtr, outSize);

    if (!ok) {
        throw new Error(new TextDecoder().decode(output));
    }

    return output;
}

/**
 * @param {{buffer: ArrayBufferLike;}} memory
 * @param {number} ptr
 * @param {Uint8Array} data
 */
function storeData(memory, ptr, data) {
    const region = { ptr, size: data.byteLength };

    new Uint8Array(memory.buffer, region.ptr, region.size).set(data);

    return region;
}

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
