export type Opaque<BaseType, BrandType = unknown> = BaseType & {
  readonly [Symbols.base]: BaseType;
  readonly [Symbols.brand]: BrandType;
};

// deno-lint-ignore no-namespace
namespace Symbols {
  export declare const base: unique symbol;
  export declare const brand: unique symbol;
}

export interface SafeDynamicLibrary<S extends SafeForeignLibraryInterface> {
  /** All of the registered library along with functions for calling them */
  symbols: StaticForeignLibraryInterface<S>;
  close(): void;
}

/** All plain number types for interfacing with foreign functions */
type NativeNumberType =
  | "u8"
  | "i8"
  | "u16"
  | "i16"
  | "u32"
  | "i32"
  | "f32"
  | "f64";

/** All BigInt number type sfor interfacing with foreign functions */
type NativeBigIntType =
  | "u64"
  | "i64"
  | "usize"
  | "isize";

type NativePointerType = "pointer";

type NativeFunctionType = "function";

type NativeVoidType = "void";

type NativeSafePointerType<B extends string = string> = Opaque<"pointer", B>;

/** All possible types for interfacing with foreign functions */
export type NativeType =
  | NativeNumberType
  | NativeBigIntType
  | NativePointerType
  | NativeSafePointerType
  | NativeFunctionType;

export type NativeResultType = NativeType | NativeVoidType;

type ToNativeTypeMap =
  & Record<NativeNumberType, number>
  & Record<NativeBigIntType, bigint | number>
  & Record<NativePointerType, TypedArray | bigint | null>
  & Record<NativeFunctionType, bigint | null>;

/** Type conversion for foreign symbol parameters and unsafe callback return types */
type ToNativeType<T extends NativeType = NativeType> = T extends
  NativeSafePointerType<infer B> ? Opaque<ToNativeTypeMap[NativePointerType], B>
  : ToNativeTypeMap[T];

type ToNativeResultTypeMap = ToNativeTypeMap & Record<NativeVoidType, void>;

/** Type conversion for unsafe callback return types */
type ToNativeResultType<T extends NativeResultType = NativeResultType> =
  T extends NativeSafePointerType<infer B>
    ? Opaque<ToNativeResultTypeMap[NativePointerType], B>
    : ToNativeResultTypeMap[T];

type ToNativeParameterTypes<T extends readonly NativeType[]> =
  //
  [(T[number])[]] extends [T] ? ToNativeType<T[number]>[]
    : [readonly (T[number])[]] extends [T] ? readonly ToNativeType<T[number]>[]
    : T extends readonly [...NativeType[]] ? {
        [K in keyof T]: ToNativeType<T[K]>;
      }
    : never;

type FromNativeTypeMap =
  & Record<NativeNumberType, number>
  & Record<NativeBigIntType, bigint>
  & Record<NativePointerType, bigint>
  & Record<NativeFunctionType, bigint>;

/** Type conversion for foreign symbol return types and unsafe callback parameters */
type FromNativeType<T extends NativeType = NativeType> = T extends
  NativeSafePointerType<infer B>
  ? Opaque<FromNativeTypeMap[NativePointerType], B>
  : FromNativeTypeMap[T];

type FromNativeResultTypeMap =
  & FromNativeTypeMap
  & Record<NativeVoidType, void>;

/** Type conversion for foregin symbol return types */
type FromNativeResultType<T extends NativeResultType = NativeResultType> =
  T extends NativeSafePointerType<infer B>
    ? Opaque<FromNativeResultTypeMap[NativePointerType], B>
    : FromNativeResultTypeMap[T];

type FromNativeParameterTypes<
  T extends readonly NativeType[],
> =
  //
  [(T[number])[]] extends [T] ? FromNativeType<T[number]>[]
    : [readonly (T[number])[]] extends [T]
      ? readonly FromNativeType<T[number]>[]
    : T extends readonly [...NativeType[]] ? {
        [K in keyof T]: FromNativeType<T[K]>;
      }
    : never;

/** A foreign function as defined by its parameter and result types */
export interface ForeignFunction<
  Parameters extends readonly NativeType[] = readonly NativeType[],
  Result extends NativeResultType = NativeResultType,
  NonBlocking extends boolean = boolean,
> {
  /** Name of the symbol, defaults to the key name in symbols object. */
  name?: string;
  parameters: Parameters;
  result: Result;
  /** When true, function calls will run on a dedicated blocking thread and will return a Promise resolving to the `result`. */
  nonblocking?: NonBlocking;
  /** When true, function calls can safely callback into JS or trigger a GC event. Default is `false`. */
  callback?: boolean;
}

export interface ForeignStatic<Type extends NativeType = NativeType> {
  /** Name of the symbol, defaults to the key name in symbols object. */
  name?: string;
  type: Type;
}

/** A foreign library interface descriptor */
export interface SafeForeignLibraryInterface {
  [name: string]: ForeignFunction | ForeignStatic;
}

/** Infers a foreign symbol */
type StaticForeignSymbol<T extends ForeignFunction | ForeignStatic> = T extends
  ForeignFunction ? FromForeignFunction<T>
  : T extends ForeignStatic ? FromNativeType<T["type"]>
  : never;

type FromForeignFunction<T extends ForeignFunction> = T["parameters"] extends
  readonly [] ? () => StaticForeignSymbolReturnType<T>
  : (
    ...args: ToNativeParameterTypes<T["parameters"]>
  ) => StaticForeignSymbolReturnType<T>;

type StaticForeignSymbolReturnType<T extends ForeignFunction> =
  ConditionalAsync<T["nonblocking"], FromNativeResultType<T["result"]>>;

type ConditionalAsync<IsAsync extends boolean | undefined, T> = IsAsync extends
  true ? Promise<T> : T;

/** Infers a foreign library interface */
type StaticForeignLibraryInterface<T extends SafeForeignLibraryInterface> = {
  [K in keyof T]: StaticForeignSymbol<T[K]>;
};

type TypedArray =
  | Int8Array
  | Uint8Array
  | Int16Array
  | Uint16Array
  | Int32Array
  | Uint32Array
  | Uint8ClampedArray
  | Float32Array
  | Float64Array
  | BigInt64Array
  | BigUint64Array;
