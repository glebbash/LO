export type Opaque<BaseType, BrandType = unknown> = BaseType & {
  readonly [Symbols.base]: BaseType;
  readonly [Symbols.brand]: BrandType;
};

export type Pointer<T> = Opaque<bigint, T>;
export type FnPointer<T> = Pointer<T>;
export type StructPointer<T> = Pointer<T>;

// deno-lint-ignore no-namespace
namespace Symbols {
  export declare const base: unique symbol;
  export declare const brand: unique symbol;
}
