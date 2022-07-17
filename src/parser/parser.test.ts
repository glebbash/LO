import { m } from "https://raw.githubusercontent.com/glebbash/multiline-str/master/src/multiline-str.ts";
import {
  assertEquals,
  assertThrows,
} from "https://deno.land/std@0.123.0/testing/asserts.ts";

import { parse } from "./parser.ts";

Deno.test("it parses empty list", () => {
  const res = parse("()");

  assertEquals(res, [[]]);
});

Deno.test("it parses symbols", () => {
  assertEquals(parse("(a)"), [["a"]]);
  assertEquals(parse("(abc)"), [["abc"]]);
});

Deno.test("it parses numbers", () => {
  assertEquals(parse("(0)"), [["0"]]);
  assertEquals(parse("(10)"), [["10"]]);
  assertEquals(parse("(10.0)"), [["10.0"]]);
  assertEquals(parse("(0.10)"), [["0.10"]]);
  assertEquals(parse("(100_000.000_000)"), [["100_000.000_000"]]);
});

Deno.test("it parses strings", () => {
  assertEquals(
    parse('("The quick brown fox jumps over the lazy dog.")'),
    [['"The quick brown fox jumps over the lazy dog."']],
  );
  assertEquals(parse('("\\"escaped quotes supported\\"")'), [
    ['"\\"escaped quotes supported\\""'],
  ]);
});

Deno.test("it parses lists with multiple atoms", () => {
  const res = parse('(abc1 123 "string\\"s")');

  assertEquals(res, [["abc1", "123", '"string\\"s"']]);
});

Deno.test("it skips whitespace and comments", () => {
  const res = parse(m`
      (a             b               c) ;comment
      ; comment
      (
        1 ; comment
        2 ; comment ; comment ; comment
        3 ;; comment
      )
      `);

  assertEquals(res, [
    ["a", "b", "c"],
    ["1", "2", "3"],
  ]);
});

Deno.test("it parses complex expressions", () => {
  const res = parse("(a (b (c () d e f (g h)))) (i) (j k (l m n))");

  assertEquals(res, [
    ["a", ["b", ["c", [], "d", "e", "f", ["g", "h"]]]],
    ["i"],
    ["j", "k", ["l", "m", "n"]],
  ]);
});

Deno.test("it parses llvm hello world", () => {
  const res = parse(m`
    ;; Hello World example

    (llvm/target-triple "x86_64-pc-linux-gnu") ; optional

    (external-fn puts (&i8) i32)

    (fn main () i32
      (puts "Hello World!")
      0
    )
    `);

  assertEquals(res, [
    ["llvm/target-triple", '"x86_64-pc-linux-gnu"'],
    ["external-fn", "puts", ["&i8"], "i32"],
    ["fn", "main", [], "i32", ["puts", '"Hello World!"'], "0"],
  ]);
});

Deno.test("it throws error on invalid input", () => {
  assertThrows(
    () => parse(""),
    Error,
    `Unexpected character: '<EOF>' at line 1, col 1`,
  );
  assertThrows(
    () => parse("("),
    Error,
    `Unexpected character: '<EOF>' at line 1, col 2`,
  );
  assertThrows(
    () => parse("(a ()"),
    Error,
    `Unexpected character: '<EOF>' at line 1, col 6`,
  );
  assertThrows(
    () => parse("(a ))"),
    Error,
    "Unexpected character: ')' at line 1, col 5",
  );
});

Deno.test("it reports errors in multiline sources", () => {
  const src = m`
      (fn main () i32
        (puts "Hello World!)
        0
      )
      `;

  assertThrows(
    () => parse(src),
    Error,
    "Unexpected character: '(' at line 2, col 3",
  );
});
