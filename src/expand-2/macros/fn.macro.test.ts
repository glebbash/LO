import { m } from "https://raw.githubusercontent.com/glebbash/multiline-str/master/src/multiline-str.ts";
import { assertEquals } from "https://deno.land/std@0.123.0/testing/asserts.ts";

import { parse, SExpr } from "../../parser/parser.ts";
import { ExpandContext, expandExpr } from "../expand.ts";
import { fnMacro } from "./fn.macro.ts";
import { unfoldMacro } from "./unfold.macro.ts";
import { valueMacro } from "./value.macro.ts";

Deno.test("it works with simple cases", () => {
  const exprs = parse(m`
    (#fn #main (a b c)
      (a a a)
      (b b b)
      (c c c)
    )

    (#main 1 2 3)
  `);

  const expanded = parse(m`
    (1 1 1)
    (2 2 2)
    (3 3 3)
  `);

  assertEquals(expand(exprs), expanded);
});

Deno.test("it works other args", () => {
  const exprs = parse(m`
    (#fn #main (a b c :other other)
      (a a a)
      (b b b)
      (c c c)
      other
    )

    (#main 1 2 3 4 5 6 7 8 9)
  `);

  const expanded = parse(m`
    (1 1 1)
    (2 2 2)
    (3 3 3)
    (4 5 6 7 8 9)
  `);

  assertEquals(expand(exprs), expanded);
});

Deno.test("it works empty other args", () => {
  const exprs = parse(m`
    (#fn #main (a b c :other args)
      (a a a)
      (b b b)
      (c c c)
      (args)
    )

    (#main 1 2 3)
  `);

  const expanded = parse(m`
    (1 1 1)
    (2 2 2)
    (3 3 3)
    (())
  `);

  assertEquals(expand(exprs), expanded);
});

Deno.test("it works single other args", () => {
  const exprs = parse(m`
    (#fn #main (:other args)
      (#unfold args)
    )

    (#main
      (call x)
      (call y)
    )
  `);

  const expanded = parse(m`
    (call x)
    (call y)
  `);

  assertEquals(expand(exprs), expanded);
});

Deno.test("it does not leak macros", () => {
  const exprs = parse(m`
    (#fn #main (a)
      (#def value a)
      value
    )

    ((#main 1))

    (value)
  `);

  const expanded = parse(m`
    (1)
    (value)
  `);

  assertEquals(expand(exprs), expanded);
});

Deno.test("it supports nested macros", () => {
  const exprs = parse(m`
    (#fn #a ()
      (a (#b))
    )

    (#fn #b ()
      b (#c)
    )

    (#fn #c ()
      c
    )

    (#a)
  `);

  const expanded = parse(m`
    (a b c)
  `);

  assertEquals(expand(exprs), expanded);
});

function expand(exprs: SExpr[]) {
  const ctx: ExpandContext = {
    level: 0,
    path: ".",
    macros: {
      "#fn": fnMacro,
      "#unfold": unfoldMacro,
      "#def": valueMacro,
    },
  };
  return expandExpr({ ctx, expr: exprs }).result[0];
}
