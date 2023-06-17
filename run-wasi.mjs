import { readFile } from "node:fs/promises";
import { WASI } from "node:wasi";
import { argv, env } from "node:process";

await runWASI(argv[2]);

async function runWASI(wasiModulePath) {
    const wasi = new WASI({
        version: "preview1",
        args: argv.slice(2),
        env,
    });

    const wasm = await WebAssembly.compile(
        await readFile(new URL(wasiModulePath, import.meta.url))
    );
    const instance = await WebAssembly.instantiate(
        wasm,
        wasi.getImportObject()
    );

    wasi.start(instance);
}
