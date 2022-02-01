import { expand } from "./expand.ts";
import { assertEquals } from "https://deno.land/std@0.123.0/testing/asserts.ts";

Deno.test("it works", () => {
  assertEquals(expand([]), []);
  assertEquals(expand(["1"]), ["1"]);
  assertEquals(expand(["1", "2"]), ["1", "2"]);
  assertEquals(expand(["1", ["2", "3"]]), ["1", ["2", "3"]]);
  assertEquals(expand([["alias", "a", "1"]]), []);
  assertEquals(
    expand([
      ["alias", "a", "1"],
      "a",
      "a",
    ]),
    ["1", "1"],
  );
});
