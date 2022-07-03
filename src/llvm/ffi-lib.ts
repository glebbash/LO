// deno-lint-ignore-file ban-types no-explicit-any

export function wrap<
  T extends Record<string, ExternalFunctionConfig<Function>>,
>(
  lib: T,
  libPath: string,
): WrappedLib<T> {
  const foreignFunctions = Object.fromEntries(
    Object.values(lib).map((
      { name, type: [result, parameters] },
    ): [string, Deno.ForeignFunction] => [name, {
      parameters,
      result,
    }]),
  );

  const dynLib = Deno.dlopen(libPath, foreignFunctions);

  const wrappedFunctions = Object.fromEntries(
    Object.entries(lib).map((
      [fnName, { name, wrap }],
    ) => [fnName, wrap((dynLib as any).symbols[name])]),
  );

  return {
    ...wrappedFunctions,
    close: () => dynLib.close(),
  } as unknown as WrappedLib<T>;
}

type WrappedLib<T extends Record<string, ExternalFunctionConfig<Function>>> =
  & {
    [P in keyof T]: GetWrappedFunction<T[P]>;
  }
  & { close: () => void };

type GetWrappedFunction<T> = T extends ExternalFunctionConfig<infer T> ? T
  : never;

export type ExternalFunctionConfig<T extends Function> = {
  name: string;
  type: [Deno.NativeResultType, Deno.NativeType[]];
  wrap: (call: (...args: unknown[]) => any) => T;
};
