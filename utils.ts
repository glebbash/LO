#!/usr/bin/env -S node

import process from "node:process";
import { test, describe } from "node:test";
import assert from "node:assert";
import fs from "node:fs/promises";
import { WASI, type WASIOptions } from "./wasi-shim.mjs";

const COMPILER_PATH = "lo.wasm";

await main();

async function main() {
    process.chdir(import.meta.dirname!);

    if (process.argv[2] === "test") {
        return commandTest();
    }

    if (process.argv[2] === "run") {
        return commandRun(process.argv.slice(3));
    }

    if (process.argv[2] === "debugWasi") {
        return commandDebugWasi(process.argv.slice(3));
    }

    if (process.argv[2] === "runWasi") {
        const filePath = new URL(process.argv[3], import.meta.url);
        const input = await fs.readFile(filePath);
        return runWASI(input, {
            args: process.argv.slice(3),
            env: process.env,
            preopens: { ".": "." },
        });
    }

    return runWASI(await fs.readFile(COMPILER_PATH), {
        preopens: { ".": "." },
        args: ["lo", ...process.argv.slice(2)],
        returnOnExit: false,
    });
}

async function commandRun(compilerArgs: string[]) {
    let programArgs = [] as string[];

    const programArgsStart = compilerArgs.indexOf("--");
    if (programArgsStart !== -1) {
        programArgs = compilerArgs.slice(programArgsStart + 1);
        compilerArgs = compilerArgs.slice(0, programArgsStart);
    }

    const stdout = new WASI.VirtualFD();
    const exitCode = await runWASI(await fs.readFile(COMPILER_PATH), {
        stdout,
        preopens: { ".": "." },
        args: ["lo", "compile", ...compilerArgs],
    });

    if (exitCode !== 0) {
        console.error("Compilation failed, see compiler error(s) above");
        process.exit(1);
    }

    const program = stdout.flushAndRead();
    await runWASI(program, {
        preopens: { ".": "." },
        returnOnExit: false,
        args: ["main.lo", ...programArgs],
    });
}

/**
 * Start an http server that runs provided WASI module for debugging with Dev Tools
 */
async function commandDebugWasi(args: string[]) {
    const filePath = new URL(args[0], import.meta.url);

    const http = await import("node:http");
    http.createServer(async (req, res) => {
        if (req.method === "GET" && req.url === "/") {
            res.setHeader("Content-Type", "text/html");
            res.end(`
                <script type="module">
                    import { init, WASI } from "https://esm.sh/@wasmer/wasi@1.2.2";

                    const compilerModule = await WebAssembly.compile(
                        await fetch('./index.wasm').then((r) => r.arrayBuffer())
                    );

                    await init();
                    const wasi = new WASI({});
                    await wasi.instantiate(compilerModule, {});
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
            const wasmFile = await fs.readFile(filePath);
            res.setHeader("Content-Type", "application/wasm");
            res.end(wasmFile);
            return;
        }

        res.statusCode = 404;
        res.end("Not Found");
    }).listen(6969, () => {
        console.log("Debug server running at http://localhost:6969/");
    });
}

async function commandTest() {
    const v1Run = await loadCompiler(await fs.readFile(COMPILER_PATH));
    const v1 = (fileName = "-i") => v1Run(["compile", fileName]);

    testVersions("compiles 42.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/42.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 42);
    });

    testVersions("compiles add.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/add.lo");

        const program = await loadWasm(output);

        assert.strictEqual(program.add(2, 3), 5);
    });

    testVersions("compiles factorial.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/factorial.lo");

        const program = await loadWasm(output);
        const result = program.factorial(5);

        assert.strictEqual(result, 120);
    });

    testVersions("compiles include.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/include.lo");

        const program = await loadWasm(output);

        assert.strictEqual(program.main(), 120);
    });

    testVersions("compiles hex-and-shifts.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/hex-and-shifts.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 31);
    });

    testVersions("compiles else-if.lo", { v1 }, async (compile) => {
        const output = await compile("examples/test/else-if.lo");

        const program = await loadWasm(output);

        assert.strictEqual(program.main(), 13);
    });

    testVersions("compiles import.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/import.lo");

        const logs = [] as unknown[];
        const program = await loadWasm(output, {
            utils: { debug: (x: unknown) => logs.push(x) },
        });

        program.main();
        assert.deepEqual(logs, [1, 2, 3]);
    });

    testVersions("compiles hello-world-raw.lo", { v1 }, async (compile) => {
        const program = await compile(
            "./examples/test/demos/hello-world-raw.lo"
        );

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(output, "Hello World!\n");
    });

    testVersions("compiles locals.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/locals.lo");

        const program = await loadWasm(output);
        assert.deepEqual(program.sub(5, 3), 2);
    });

    testVersions("compiles globals.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/globals.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 69);
    });

    testVersions("compiles loop.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/loop.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 120);
    });

    testVersions("compiles for-loop.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/for-loop.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 138);
    });

    testVersions("compiles methods.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/methods.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 1);
    });

    testVersions("compiles nested-if-break.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/nested-if-break.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 1);
    });

    testVersions("compiles struct.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/struct.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 13);
    });

    testVersions("compiles auto-forward-decl.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/auto-forward-decl.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 42);
    });

    testVersions(
        "compiles struct-value-field-access.lo",
        { v1 },
        async (compile) => {
            const output = await compile(
                "./examples/test/struct-value-field-access.lo"
            );

            const program = await loadWasm(output);
            const result = program.main();

            assert.strictEqual(result, 18);
        }
    );

    testVersions("compiles decl-nesting.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/decl-nesting.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 16);
    });

    testVersions("compiles struct-ref.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/struct-ref.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 3);
    });

    testVersions("compiles macro.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/macro.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 16);
    });

    testVersions("compiles wasi.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/lib/wasi.lo");

        const wasi = new WASI({
            version: "preview1",
            sysCalls: await WASI.NodeSysCalls(),
        });
        const wasm = await WebAssembly.compile(output);
        await WebAssembly.instantiate(wasm, wasi.getImportObject());
    });

    testVersions("compiles string-pooling.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/string-pooling.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 13);
    });

    testVersions("compiles zst-noop.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/zst-noop.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, undefined);
    });

    testVersions("compiles std.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/std.test.lo");

        await loadWasm(output);
    });

    testVersions("compiles vec.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/vec.test.lo");
        const lib = await loadWasm(output);

        const vec = lib.vec_new(4, 1);

        lib.vec_push_u8(vec, 1);

        lib.vec_push_u8(vec, 3);
        lib.vec_push_u8(vec, 2);
        lib.vec_swap(vec, 1, 2);

        storeData(lib.memory, 1000, new Uint8Array([4, 5]));
        lib.vec_push_all(vec, 1000, 2);

        storeData(lib.memory, 1000, new Uint8Array([6]));
        lib.vec_push_all(vec, 1000, 1);

        lib.vec_push_u8(vec, 7);

        assert.strictEqual(lib.vec_get_u8(vec, 0), 1);
        assert.strictEqual(lib.vec_get_u8(vec, 1), 2);
        assert.strictEqual(lib.vec_get_u8(vec, 2), 3);
        assert.strictEqual(lib.vec_get_u8(vec, 3), 4);
        assert.strictEqual(lib.vec_get_u8(vec, 4), 5);
        assert.strictEqual(lib.vec_get_u8(vec, 5), 6);
        assert.strictEqual(lib.vec_get_u8(vec, 6), 7);
        assert.strictEqual(lib.vec_len(vec), 7);

        function storeData(
            memory: WebAssembly.Memory,
            ptr: number,
            data: Uint8Array
        ) {
            const region = { ptr, size: data.byteLength };

            new Uint8Array(memory.buffer, region.ptr, region.size).set(data);

            return region;
        }
    });

    testVersions("compiles hello-world.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/demos/hello-world.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(output, "Hello World!\n");
    });

    testVersions("compiles echo.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/demos/echo.lo");

        const stdin = new WASI.VirtualFD();
        stdin.write(new TextEncoder().encode("abc"));
        stdin.flush();

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdin, stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(output, "abc");
    });

    testVersions("compiles args.test.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/args.test.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout, args: ["123", "456", "789"] });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(output, "123\n456\n789\n");
    });

    testVersions("compiles cat.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/demos/cat.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, {
            stdout,
            args: ["cat.lo", "examples/test/42.lo"],
            preopens: { ".": "." },
        });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            await fs.readFile("examples/test/42.lo", "utf-8")
        );
    });

    testVersions("compiles tracing.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/tracing.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            "examples/test/tracing.lo:4:13 - hello there\n"
        );
    });

    testVersions("compiles struct-in-struct.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/struct-in-struct.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(output, "3\n3\n3\n3\n3\n3\n3\n");
    });

    testVersions("compiles heap-alloc.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/heap-alloc.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            m`
            p1 = 1048600
            p2 = 1048600
            p3 = 1048612

            `
        );
    });

    testVersions("compiles defer.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/defer.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            m`
            defer(inner_fn): loop (iteration #1)
            defer(inner_fn): loop (iteration #2)
            defer(inner_fn): loop (iteration #3)
            defer(inner_fn): top level
            defer(main): 3
            defer(main): 2
            defer(main): 1

            `
        );
    });

    testVersions("compiles errors.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/errors.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            m`
            10 / 5 = 2, remainder = 0
            10 / 3 = 3, remainder = 1
            10 / 0 is undefined

            `
        );
    });

    testVersions("compiles do-with.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/do-with.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            m`
            123456789
            -------------
            { "type": "file", "index": 1, "path": "some_path.lo" },

            `
        );
    });

    testVersions("compiles enums.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/enums.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            m`
            +a

            `
        );
    });

    testVersions("compiles if-match-chain.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/if-match-chain.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            m`
            going left 5 steps
            going right 3 steps

            `
        );
    });

    testVersions(
        "compiler reports multiple errors in fault-tolerance.lo",
        { v1 },
        async (compile) => {
            try {
                await compile("./examples/test/fault-tolerance.lo");
            } catch (err) {
                assert.strictEqual(
                    (err as Error).message,
                    m`
                    ERROR: examples/test/fault-tolerance.lo:10:4 - Cannot redefine main, already defined at examples/test/fault-tolerance.lo:2:4
                    ERROR: examples/test/fault-tolerance.lo:2:17 - Duplicate function parameter name: a
                    ERROR: examples/test/fault-tolerance.lo:5:9 - Cannot redefine local x, previously defined at examples/test/fault-tolerance.lo:3:9
                    ERROR: examples/test/fault-tolerance.lo:13:9 - Cannot redefine local x, previously defined at examples/test/fault-tolerance.lo:11:9

                    `
                );
            }
        }
    );

    testVersions(
        "inspect emits partial diagnostics and multiple errors in fault-tolerance.lo",
        { v1: v1Run },
        async (run) => {
            const stdout = new WASI.VirtualFD();
            try {
                await run(["inspect", "./examples/test/fault-tolerance.lo"], {
                    stdout,
                });
            } catch (err) {
                // stderr is empty
                assert.strictEqual((err as Error).message, "");

                // stdout contains partial diagnostics and errors in json format
                assert.strictEqual(
                    stdout.flushAndReadUtf8(),
                    m`
                    [
                    { "type": "file", "index": 1, "path": "examples/test/fault-tolerance.lo" },
                    { "type": "message", "content": "Cannot redefine main, already defined at examples/test/fault-tolerance.lo:2:4", "severity": "error", "loc": "1/10:4-10:8" },
                    { "type": "message", "content": "Duplicate function parameter name: a", "severity": "error", "loc": "1/2:17-2:23" },
                    { "type": "info", "hover": "let x: u32", "loc": "1/3:9-3:10" },
                    { "type": "message", "content": "Cannot redefine local x, previously defined at examples/test/fault-tolerance.lo:3:9", "severity": "error", "loc": "1/5:9-5:10" },
                    { "type": "info", "hover": "let y: u32", "loc": "1/6:9-6:10" },
                    { "type": "info", "link": "1/3:9-3:10", "hover": "let x: u32", "loc": "1/6:13-6:14" },
                    { "type": "info", "hover": "let x: u32", "loc": "1/11:9-11:10" },
                    { "type": "message", "content": "Cannot redefine local x, previously defined at examples/test/fault-tolerance.lo:11:9", "severity": "error", "loc": "1/13:9-13:10" },
                    { "type": "info", "hover": "let y: u32", "loc": "1/14:9-14:10" },
                    { "type": "info", "link": "1/11:9-11:10", "hover": "let x: u32", "loc": "1/14:13-14:14" },
                    { "type": "end" }
                    ]

                    `
                );
            }
        }
    );

    describe("<stdin> input", () => {
        const v1 = v1Run;

        testVersions("compiles 42.lo", { v1 }, async (run) => {
            const stdin = new WASI.VirtualFD();
            stdin.write(await fs.readFile("./examples/test/42.lo"));
            stdin.flush();

            const output = await run(["compile", "-i"], { stdin });

            const program = await loadWasm(output);
            const result = program.main();

            assert.strictEqual(result, 42);
        });
    });

    describe("aoc", () => {
        testVersions("compiles 2020 day 1", { v1 }, async (compile) => {
            const part1 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/1.lo"
            );
            assert.strictEqual(part1, "157059\n");

            const part2 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/1-part2.lo"
            );
            assert.strictEqual(part2, "165080960\n");
        });

        testVersions("compiles 2020 day 2", { v1 }, async (compile) => {
            const part1 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/2.lo"
            );
            assert.strictEqual(part1, "560\n");

            const part2 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/2-part2.lo"
            );
            assert.strictEqual(part2, "303\n");
        });

        testVersions("compiles 2020 day 3", { v1 }, async (compile) => {
            const part1 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/3.lo"
            );
            assert.strictEqual(part1, "151\n");

            const part2 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/3-part2.lo"
            );
            assert.strictEqual(part2, "7540141059\n");
        });

        testVersions("compiles 2020 day 4", { v1 }, async (compile) => {
            const part1 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/4.lo"
            );
            assert.strictEqual(part1, "264\n");

            const part2 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/4-part2.lo"
            );
            assert.strictEqual(part2, "224\n");
        });

        testVersions("compiles 2020 day 5", { v1 }, async (compile) => {
            const part1 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/5.lo"
            );
            assert.strictEqual(part1, "947\n");

            const part2 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/5-part2.lo"
            );
            assert.strictEqual(part2, "636\n");
        });

        testVersions("compiles 2023 day 1", { v1 }, async (compile) => {
            const part1 = await runAoc(
                compile,
                "./examples/test/demos/aoc2023/1.lo"
            );
            assert.strictEqual(part1, "54450\n");

            const part2 = await runAoc(
                compile,
                "./examples/test/demos/aoc2023/1-part2.lo"
            );
            assert.strictEqual(part2, "54265\n");
        });

        async function runAoc(
            compile: (sourcePath: string) => Promise<Uint8Array>,
            path: string
        ) {
            const program = await compile(path);

            const stdout = new WASI.VirtualFD();
            const exitCode = await runWASI(program, {
                stdout,
                preopens: { ".": "." },
            });

            if (exitCode !== 0) {
                throw new Error(`Process exited with error code: ${exitCode}`);
            }

            return stdout.flushAndReadUtf8();
        }
    });

    describe("wasm4", () => {
        const wasm4Imports = {
            env: new Proxy(
                { memory: new WebAssembly.Memory({ initial: 1 }) },
                {
                    get: (target, prop) =>
                        target[prop as keyof typeof target] ?? (() => void 0),
                }
            ),
        };

        testVersions("compiles blink.lo", { v1 }, async (compile) => {
            const output = await compile(
                "./examples/test/demos/wasm4/src/blink.lo"
            );
            await loadWasm(output, wasm4Imports);
        });

        testVersions("compiles dark-maze.lo", { v1 }, async (compile) => {
            const output = await compile(
                "./examples/test/demos/wasm4/src/dark-maze.lo"
            );
            await loadWasm(output, wasm4Imports);
        });

        testVersions("compiles slasher.lo", { v1 }, async (compile) => {
            const output = await compile(
                "./examples/test/demos/wasm4/src/slasher.lo"
            );
            await loadWasm(output, wasm4Imports);
        });
    });

    describe("formatter", async () => {
        if (process.argv.includes("--skip-fmt")) {
            return;
        }

        const format = async (fileName = "-i") => {
            const output = await v1Run(["format", fileName]);
            return new TextDecoder().decode(output);
        };

        const formattedFiles = await fs
            .readdir("examples", { recursive: true })
            .then((files) => files.filter((f) => f.endsWith(".lo")))
            .then((files) => files.map((f) => `examples/${f}`));

        for (const fileName of formattedFiles) {
            test(`formats ${fileName}`, async () => {
                const formatted = await format(fileName);
                const expected = await fs.readFile(fileName, "utf8");

                // formatting the file to update snapshot
                await fs.writeFile(fileName, formatted);

                assert.strictEqual(formatted, expected);
            });
        }
    });

    if (process.argv.includes("--fast")) {
        return;
    }

    describe("self-hosted", async () => {
        const sHRun = await loadCompiler(
            await v1("examples/self-hosted/_bin.lo")
        );

        describe("lexer", async () => {
            const lexV1 = async (fileName = "-i") => {
                const output = await v1Run(["lex", fileName]);
                return new TextDecoder().decode(output);
            };
            const lexSH = async (fileName = "-i") => {
                const output = await sHRun(["lex", fileName]);
                return new TextDecoder().decode(output);
            };

            const filesToLex = await fs
                .readdir("examples", { recursive: true })
                .then((files) => files.filter((f) => f.endsWith(".lo")))
                .then((files) => files.map((f) => `examples/${f}`))
                // TODO: look into printing performance
                .then((files) => files.filter((f) => !f.includes("dark-maze")));

            for (const fileName of filesToLex) {
                test(`correctly lexes ${fileName}`, async () => {
                    const v1 = await lexV1(fileName);
                    const sh = await lexSH(fileName);

                    assert.equal(sh, v1);
                });
            }
        });
    });

    describe("interpreter", () => {
        const interpret = (fileName = "-i") => v1Run(["eval", fileName]);

        test("interprets 42.lo", async () => {
            const res = await interpret("examples/test/42.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 42\n"
            );
        });

        test("interprets include.lo", async () => {
            const res = await interpret("examples/test/include.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 120\n"
            );
        });

        test("interprets else-if.lo", async () => {
            const res = await interpret("examples/test/else-if.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 13\n"
            );
        });

        test("interprets globals.lo", async () => {
            const res = await interpret("examples/test/globals.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 69\n"
            );
        });

        test("interprets hex-and-shifts.lo", async () => {
            const res = await interpret("examples/test/hex-and-shifts.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 31\n"
            );
        });

        test("interprets loop.lo", async () => {
            const res = await interpret("./examples/test/loop.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 120\n"
            );
        });

        test("interprets for-loop.lo", async () => {
            const res = await interpret("./examples/test/for-loop.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 138\n"
            );
        });

        test("interprets methods.lo", async () => {
            const res = await interpret("./examples/test/methods.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 1\n"
            );
        });

        test("interprets decl-nesting.lo", async () => {
            const res = await interpret("./examples/test/decl-nesting.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 16\n"
            );
        });

        test("interprets struct.lo", async () => {
            const res = await interpret("./examples/test/struct.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 13\n"
            );
        });

        test("interprets nested-if-break.lo", async () => {
            const res = await interpret("./examples/test/nested-if-break.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 1\n"
            );
        });

        test("interprets struct-ref.lo", async () => {
            const res = await interpret("./examples/test/struct-ref.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 3\n"
            );
        });

        test("interprets macro.lo", async () => {
            const res = await interpret("./examples/test/macro.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 16\n"
            );
        });

        test("interprets import.lo", async () => {
            const res = await interpret("./examples/test/import.lo");
            // NOTE: can't really see debug output because it's on stderr and that is ignored on exit = 0
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: \n"
            );
        });

        test("interprets string-pooling.lo", async () => {
            const res = await interpret("./examples/test/string-pooling.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "result of `main` is: 13\n"
            );
        });

        // wasi

        test("interprets hello-world-raw.lo", async () => {
            const res = await interpret(
                "./examples/test/demos/hello-world-raw.lo"
            );
            assert.strictEqual(new TextDecoder().decode(res), "Hello World!\n");
        });

        test("interprets hello-world.lo", async () => {
            const res = await interpret("./examples/test/demos/hello-world.lo");
            assert.strictEqual(new TextDecoder().decode(res), "Hello World!\n");
        });

        test("interprets echo.lo", async () => {
            const res = await interpret("./examples/test/demos/echo.lo");
            // NOTE: no stdin provided so no output
            assert.strictEqual(new TextDecoder().decode(res), "");
        });

        test("interprets args.test.lo", async () => {
            const res = await interpret("./examples/test/args.test.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                m`
                lo
                eval
                ./examples/test/args.test.lo

                `
            );
        });

        // TODO: provide a way to pass own args to --eval
        test("interprets cat.lo", async () => {
            try {
                await interpret("./examples/test/demos/cat.lo");
            } catch (err) {
                assert.strictEqual(
                    (err as Error).message,
                    "Usage cat.lo <file>\n"
                );
            }
        });

        test("interprets tracing.lo", async () => {
            const res = await interpret("./examples/test/tracing.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "examples/test/tracing.lo:4:13 - hello there\n"
            );
        });

        test("interprets struct-in-struct.lo", async () => {
            const res = await interpret("./examples/test/struct-in-struct.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                "3\n3\n3\n3\n3\n3\n3\n"
            );
        });

        test("interprets heap-alloc.lo", async () => {
            const res = await interpret("./examples/test/heap-alloc.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                m`
                p1 = 1048600
                p2 = 1048600
                p3 = 1048612

                `
            );
        });

        test("interprets defer.lo", async () => {
            const res = await interpret("./examples/test/defer.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                m`
                defer(inner_fn): loop (iteration #1)
                defer(inner_fn): loop (iteration #2)
                defer(inner_fn): loop (iteration #3)
                defer(inner_fn): top level
                defer(main): 3
                defer(main): 2
                defer(main): 1

                `
            );
        });

        test("interprets errors.lo", async () => {
            const res = await interpret("./examples/test/errors.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                m`
                10 / 5 = 2, remainder = 0
                10 / 3 = 3, remainder = 1
                10 / 0 is undefined

                `
            );
        });

        test("interprets enums.lo", async () => {
            const res = await interpret("./examples/test/enums.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                m`
                +a

                `
            );
        });

        test("interprets if-match-chain.lo", async () => {
            const res = await interpret("./examples/test/if-match-chain.lo");
            assert.strictEqual(
                new TextDecoder().decode(res),
                m`
                going left 5 steps
                going right 3 steps

                `
            );
        });

        // aoc

        // TODO: look into performance of part 2
        test("interprets aoc2020 day 1", async () => {
            const res1 = await interpret("./examples/test/demos/aoc2020/1.lo");
            assert.strictEqual(new TextDecoder().decode(res1), "157059\n");

            const res2 = await interpret(
                "./examples/test/demos/aoc2020/1-part2.lo"
            );
            assert.strictEqual(new TextDecoder().decode(res2), "165080960\n");
        });

        test("interprets aoc2020 day 2", async () => {
            const res1 = await interpret("./examples/test/demos/aoc2020/2.lo");
            assert.strictEqual(new TextDecoder().decode(res1), "560\n");

            const res2 = await interpret(
                "./examples/test/demos/aoc2020/2-part2.lo"
            );
            assert.strictEqual(new TextDecoder().decode(res2), "303\n");
        });

        test("interprets aoc2020 day 3", async () => {
            const res1 = await interpret("./examples/test/demos/aoc2020/3.lo");
            assert.strictEqual(new TextDecoder().decode(res1), "151\n");

            const res2 = await interpret(
                "./examples/test/demos/aoc2020/3-part2.lo"
            );
            assert.strictEqual(new TextDecoder().decode(res2), "7540141059\n");
        });

        test("interprets aoc2020 day 4", async () => {
            const res1 = await interpret("./examples/test/demos/aoc2020/4.lo");
            assert.strictEqual(new TextDecoder().decode(res1), "264\n");

            const res2 = await interpret(
                "./examples/test/demos/aoc2020/4-part2.lo"
            );
            assert.strictEqual(new TextDecoder().decode(res2), "224\n");
        });

        // TODO: look into performance of part 2
        test("interprets aoc2020 day 5", async () => {
            const res1 = await interpret("./examples/test/demos/aoc2020/5.lo");
            assert.strictEqual(new TextDecoder().decode(res1), "947\n");

            const res2 = await interpret(
                "./examples/test/demos/aoc2020/5-part2.lo"
            );
            assert.strictEqual(new TextDecoder().decode(res2), "636\n");
        });

        test("interprets aoc2023 day 1", async () => {
            const res1 = await interpret("./examples/test/demos/aoc2023/1.lo");
            assert.strictEqual(new TextDecoder().decode(res1), "54450\n");

            const res2 = await interpret(
                "./examples/test/demos/aoc2023/1-part2.lo"
            );
            assert.strictEqual(new TextDecoder().decode(res2), "54265\n");
        });
    });

    function testVersions<T>(
        testName: string,
        compilers: Record<string, T>,
        testFn: (compile: T) => Promise<void>
    ) {
        for (const [compilerName, compile] of Object.entries(compilers)) {
            test(`${testName} (${compilerName})`, () => testFn(compile));
        }
    }

    async function loadCompiler(compilerWasmBinary: Uint8Array) {
        const mod = await WebAssembly.compile(compilerWasmBinary);

        return async (
            args = ["help"],
            {
                stdin = new WASI.VirtualFD(),
                stdout = new WASI.VirtualFD(),
                stderr = new WASI.VirtualFD(),
            } = {}
        ) => {
            const wasi = new WASI({
                version: "preview1",
                stdin,
                stdout,
                stderr,
                args: ["lo", ...args],
                preopens: { ".": "." },
                sysCalls: await WASI.NodeSysCalls(),
            });

            const instance = await WebAssembly.instantiate(mod, {
                ...wasi.getImportObject(),
                ...{ console: { ...console } },
            });

            try {
                const exitCode = wasi.start(instance);

                if (exitCode ?? 0 !== 0) {
                    throw new Error(stderr.flushAndReadUtf8());
                }

                return stdout.flushAndRead();
            } catch (err) {
                const errorMessage = stderr.flushAndReadUtf8();
                if (errorMessage !== "") {
                    if (
                        err instanceof WebAssembly.RuntimeError &&
                        err.message.includes("unreachable")
                    ) {
                        err.message = errorMessage;
                    } else {
                        throw new Error(errorMessage);
                    }
                }

                throw err;
            }
        };
    }
}

// utils

async function loadWasm(data: Uint8Array, imports?: WebAssembly.Imports) {
    const mod = await WebAssembly.instantiate(data, imports);
    // deno-lint-ignore no-explicit-any
    return mod.instance.exports as any;
}

async function runWASI(
    data: Uint8Array,
    wasiOptions?: Omit<WASIOptions, "version" | "sysCalls">,
    additionalImports = {}
) {
    const wasi = new WASI({
        version: "preview1",
        sysCalls: await WASI.NodeSysCalls(),
        ...wasiOptions,
    });

    const wasm = await WebAssembly.compile(data);
    const instance = await WebAssembly.instantiate(wasm, {
        ...wasi.getImportObject(),
        ...{ console: { ...console } },
        ...additionalImports,
    });

    try {
        return wasi.start(instance);
    } catch (err) {
        if (err instanceof WebAssembly.RuntimeError) {
            if (err.message.includes("unreachable")) {
                const memory = instance.exports.memory as WebAssembly.Memory;
                const [errorIndicator, errorCode] = new Uint32Array(
                    memory.buffer.slice(0, 8)
                );

                if (errorIndicator === 69420) {
                    err.message = "Abort code " + errorCode;
                }
            }
        }
        throw err;
    }
}

function m(strings: TemplateStringsArray, ...args: unknown[]) {
    const INDENT_PLACEHOLDER = "```";

    // interpolate
    const stringLines = strings
        .map(
            (str, index) =>
                str.replace(/\\\n[ \t]*/g, "").replace(/\\`/g, "`") +
                String(args[index] ?? "").replace(
                    /\n/g,
                    `\n${INDENT_PLACEHOLDER}`
                )
        )
        .join("")
        .split("\n")
        .slice(1);

    const lastLineIndentation = stringLines.pop()!.length;

    // dedent
    return stringLines
        .map((str) =>
            str
                .split(INDENT_PLACEHOLDER)
                .join(" ".repeat(lastLineIndentation))
                .slice(lastLineIndentation)
        )
        .join("\n");
}
