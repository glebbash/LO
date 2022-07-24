import { m } from "https://raw.githubusercontent.com/glebbash/multiline-str/master/src/multiline-str.ts";
import { assertEquals } from "https://deno.land/std@0.123.0/testing/asserts.ts";

import { parse, SExpr } from "../../parser/parser.ts";
import { ExpandContext, expandExpr } from "../expand.ts";
import { unfoldMacro } from "./unfold.macro.ts";
import { valueMacro } from "./value.macro.ts";

Deno.test("it can unfold list", () => {
  const exprs = parse(m`
    (#unfold (1 2 3))
  `);

  assertEquals(expand(exprs), [
    "1",
    "2",
    "3",
  ]);
});

Deno.test("it can unfold list from the macro", () => {
  const exprs = parse(m`
    (#def x (1 2 3))
    (#unfold x)
  `);

  assertEquals(expand(exprs), [
    "1",
    "2",
    "3",
  ]);
});

Deno.test("it just returns the atom it if was passed", () => {
  const exprs = parse(m`
    (#unfold 1)
  `);

  assertEquals(expand(exprs), [
    "1",
  ]);
});

Deno.test("it expands and returns the atom it if was passed as macro", () => {
  const exprs = parse(m`
    (#def x 1)
    (#unfold x)
  `);

  assertEquals(expand(exprs), [
    "1",
  ]);
});

function expand(exprs: SExpr[]) {
  const ctx: ExpandContext = {
    level: 0,
    path: ".",
    macros: {
      "#def": valueMacro,
      "#unfold": unfoldMacro,
    },
  };
  return expandExpr({ ctx, expr: exprs }).result[0];
}
