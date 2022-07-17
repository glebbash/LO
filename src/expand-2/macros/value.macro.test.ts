import { assertEquals } from "https://deno.land/std@0.123.0/testing/asserts.ts";
import { m } from "https://raw.githubusercontent.com/glebbash/multiline-str/master/src/multiline-str.ts";

import { parse, SExpr } from "../../parser/parser.ts";
import { ExpandContext, expandExpr } from "../expand.ts";
import { valueMacro } from "./value.macro.ts";

Deno.test("it defines an alias", () => {
  const exprs = parse(m`
    (#def x 1)
    (x)
  `);

  const expanded = parse(m`
    (1)
  `);

  assertEquals(expand(exprs), expanded);
});

function expand(exprs: SExpr[]) {
  const ctx: ExpandContext = {
    level: 0,
    path: ".",
    macros: {
      "#def": valueMacro,
    },
  };
  return expandExpr({ ctx, expr: exprs }).result[0];
}
