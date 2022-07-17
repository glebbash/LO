import { SExpr } from "../../parser/parser.ts";
import {
  expectArgsLengthAtLeast,
  expectList,
  expectString,
} from "../../s-expr/assertions.ts";
import { getStringValue } from "../../s-expr/transformers.ts";
import { ModuleContext } from "../compiler.ts";
import { stringToBytes } from "../utils.ts";
import { buildValueInFunctionContext } from "./mod.ts";

export function buildMemory(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
) {
  const [...segments] = expectArgsLengthAtLeast(
    1,
    args,
    command,
  );

  ctx.module.setMemory(
    1,
    256,
    "memory",
    segments.map((segmentExpr) => {
      expectList(segmentExpr);

      if (segmentExpr.length !== 2) {
        throw new Error(`${command} expects list of segments of length 2`);
      }

      const [offsetExpr, dataExpr] = segmentExpr;

      expectString(dataExpr);

      return {
        passive: false,
        // TODO: restrict to top-level constant expressions only
        offset: buildValueInFunctionContext(offsetExpr, ctx),
        data: stringToBytes(getStringValue(dataExpr)),
      };
    }),
    false,
  );
}
