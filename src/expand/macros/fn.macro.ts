import { SExpr } from "../../parser/parser.ts";
import {
  expectArgsLengthAtLeast,
  expectList,
  expectSymbol,
} from "../../s-expr/assertions.ts";
import { expandList } from "../expand.ts";
import { buildMacroFunction, buildMacroValue, Macro } from "./mod.ts";

export const fnMacro = buildMacroFunction((ctx, command, args) => {
  const [name, paramExprs, ...exprs] = expectArgsLengthAtLeast(
    3,
    args,
    command,
  );
  expectSymbol(name);
  expectList(paramExprs);

  const paramDefs = buildParamDefs(paramExprs);

  // TODO: should it be allowed to have recursive macros?
  const definedMacro = buildMacroFunction((ctx, command, args) => {
    const expandedArgs = expandList(ctx, args).result.flat();

    const argsMacros = extractArgs(
      paramDefs,
      command,
      expandedArgs,
    );

    const res = expandList({
      ...ctx,
      macros: { ...ctx.macros, ...argsMacros },
    }, exprs);

    return {
      result: res.result.flat(),
      ctx,
    };
  });

  return {
    result: [],
    ctx: {
      ...ctx,
      macros: {
        ...ctx.macros,
        [name]: definedMacro,
      },
    },
  };
});

type ParamDef = [name: string, index: number];

function buildParamDefs(params: SExpr[]): ParamDef[] {
  const paramDefs: ParamDef[] = [];

  for (let index = 0; index < params.length; index++) {
    const param = params[index];
    expectSymbol(param);

    if (param === ":other") {
      if (index + 1 !== params.length - 1) {
        throw new Error(
          "there should be one and only one name for :other param",
        );
      }

      const otherParam = params[index + 1];
      expectSymbol(otherParam);

      paramDefs.push([otherParam, -(index + 1)]);
      break;
    }

    paramDefs.push([param, index + 1]);
  }

  return paramDefs;
}

function extractArgs(
  paramDefs: ParamDef[],
  command: string,
  args: SExpr[],
): Record<string, Macro> {
  const extractedArgs: Record<string, Macro> = {};

  for (const [paramName, index] of paramDefs) {
    // handle other args
    if (index < 0) {
      extractedArgs[paramName] = buildMacroValue(args.slice(-index - 1));
      break;
    }

    if (index > args.length) {
      throw new Error(
        `${command} requires '${paramName}' argument`,
      );
    }

    extractedArgs[paramName] = buildMacroValue(args[index - 1]);
  }

  return extractedArgs;
}
