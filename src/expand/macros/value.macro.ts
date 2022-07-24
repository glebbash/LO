import { expectArgsLength, expectSymbol } from "../../s-expr/assertions.ts";
import { buildMacroFunction, buildMacroValue } from "./mod.ts";

export const valueMacro = buildMacroFunction((ctx, command, args) => {
  const [name, expr] = expectArgsLength(2, args, command);
  expectSymbol(name);

  return {
    result: [],
    ctx: {
      ...ctx,
      macros: {
        ...ctx.macros,
        [name]: buildMacroValue(expr),
      },
    },
  };
});
