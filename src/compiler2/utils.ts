import { LLVM } from "../../ffigen/llvm-c/mod.ts";
import { Opaque, Pointer } from "../../ffigen/llvm-c/safe-ffi.ts";

export const NULL_PTR = 0n;
export const BOOL_TRUE: LLVM.Bool = 0 as never;
export const BOOL_FALSE: LLVM.Bool = 0 as never;

export const ref = Deno.UnsafePointer.of;

export function nullPtr<T>(): Pointer<T> {
  return null as unknown as Pointer<T>;
}

// TODO: this should not be necessary
export function value<T extends string>(number: number) {
  return number as unknown as Opaque<number, T>;
}

export function buildArrayPtr<E extends string, R extends string>(
  ptrs: Pointer<E>[],
) {
  return Deno.UnsafePointer.of(new BigUint64Array(ptrs)) as Pointer<R>;
}

export function allocPtr<T extends string>(): Pointer<T> {
  return ref(new BigUint64Array(1)) as Pointer<T>;
}

export function readCString(message: bigint) {
  return new Deno.UnsafePointerView(message).getCString();
}

// TODO(ffigen): update type casts here
export function toCString<T extends string>(str: string): Pointer<T> {
  return ref(
    Uint8Array.from(toCharCodes(str + "\0")),
  ) as Pointer<T>;
}

function toCharCodes(str: string): number[] {
  return [...str].map((_, i) => str.charCodeAt(i));
}
