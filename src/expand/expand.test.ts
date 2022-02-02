import { expandExprs } from "./expand.ts";
import { assertEquals } from "https://deno.land/std@0.123.0/testing/asserts.ts";

Deno.test("it works", () => {
  assertEquals(expandExprs([]), []);
  assertEquals(expandExprs(["1"]), ["1"]);
  assertEquals(expandExprs(["1", "2"]), ["1", "2"]);
  assertEquals(expandExprs(["1", ["2", "3"]]), ["1", ["2", "3"]]);
  assertEquals(expandExprs([["alias", "a", "1"]]), []);
  assertEquals(
    expandExprs([
      ["alias", "a", "1"],
      "a",
      "a",
    ]),
    ["1", "1"],
  );
});
