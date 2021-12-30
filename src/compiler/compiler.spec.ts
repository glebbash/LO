import { m } from 'multiline-str';

import { parse } from '../parser/parser';
import { compile } from './compiler';

// TODO: add smaller tests

describe('compiler', () => {
  it('compiles hello world example', async () => {
    const source = m`
      ;; Hello World example

      (llvm/target-triple "x86_64-pc-linux-gnu") ; optional

      (external-fn puts (&i8) i32)

      (fn main () i32
        (puts "Hello World!")
        (i32 0)
      )
      `;
    const exprs = parse(source);

    const llvmIR = await compile(exprs);

    expect(llvmIR).toBe(m`
      ; ModuleID = 'main'
      source_filename = "main"
      target triple = "x86_64-pc-linux-gnu"

      @0 = private unnamed_addr constant [13 x i8] c"Hello World!\\00", align 1

      declare i32 @puts(i8*)

      define i32 @main() {
      entry:
        %0 = call i32 @puts(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @0, i32 0, i32 0))
        ret i32 0
      }

      `);
  });
});
