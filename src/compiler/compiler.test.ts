import { m } from "https://raw.githubusercontent.com/glebbash/multiline-str/master/src/multiline-str.ts";

import { parse } from "../parser/parser.ts";
import { buildLLVMIR, compileToModule, LLVM_PATH } from "./compiler.ts";
import { assertEquals } from "https://deno.land/std@0.123.0/testing/asserts.ts";
import { loadLLVM } from "../llvm-c-14/llvm-c/mod.ts";

Deno.test("it compiles hello world example", () => {
  const source = m`
    ;; Hello World example

    (llvm/target-triple "x86_64-pc-linux-gnu") ; optional

    (external-fn puts (&i8) i32)

    (fn main () i32
      (puts "Hello World!")
      0
    )
    `;

  const llvm = loadLLVM(LLVM_PATH);
  const module = compileToModule(parse(source), llvm);
  const llvmIR = buildLLVMIR(module);

  module.dispose();
  llvm.close();

  assertEquals(
    llvmIR,
    m`
    ; ModuleID = 'main'
    source_filename = "main"
    target triple = "x86_64-pc-linux-gnu"

    @str = private unnamed_addr constant [13 x i8] c"Hello World!\\00", align 1

    declare i32 @puts(i8*)

    define i32 @main() {
    entry:
      %0 = call i32 @puts(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @str, i32 0, i32 0))
      ret i32 0
    }

    `,
  );
});
