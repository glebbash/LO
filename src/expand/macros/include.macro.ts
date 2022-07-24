import { dirname, resolve } from "https://deno.land/std@0.123.0/path/mod.ts";

import { parse } from "../../parser/parser.ts";
import { expectArgsLength, expectString } from "../../s-expr/assertions.ts";
import { getStringValue } from "../../s-expr/transformers.ts";
import { ExpandContext, expandList } from "../expand.ts";
import { buildMacroFunction } from "./mod.ts";

export const includeMacro = buildMacroFunction((ctx, command, args) => {
  const [fileNameStr] = expectArgsLength(1, args, command);
  expectString(fileNameStr);

  if (ctx.level !== 1) {
    throw new Error(`${command} can only be used at top level`);
  }

  const filePath = getStringValue(fileNameStr);
  const fullFilePath = resolve(ctx.path, filePath);

  const fileContent = Deno.readTextFileSync(fullFilePath);
  const exprs = parse(fileContent);

  const path = dirname(fullFilePath);
  const fileCtx: ExpandContext = { ...ctx, path };
  const res = expandList(fileCtx, exprs);

  return {
    result: res.result.flat(),
    ctx: { ...ctx, macros: res.ctx.macros },
  };
});
