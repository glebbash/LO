// deno-lint-ignore-file
export type Opaque<BaseType, BrandType = unknown> = BaseType & {
  readonly [Symbols.base]: BaseType;
  readonly [Symbols.brand]: BrandType;
};

export type Pointer<T = string> = Opaque<bigint, T>;
export type FnPointer<T = string> = Pointer<T>;
export type StructPointer<T = string> = Pointer<T>;

namespace Symbols {
  export declare const base: unique symbol;
  export declare const brand: unique symbol;
}
