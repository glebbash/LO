import { LLVM } from "../llvm-c-14/llvm-c/mod.ts";
import { Pointer } from "../llvm-c-14/llvm-c/safe-ffi.ts";

export const NULL_PTR = 0n;
export const BOOL_TRUE: LLVM.Bool = 0;
export const BOOL_FALSE: LLVM.Bool = 0;

export const ref = Deno.UnsafePointer.of;

export function buildArrayPtr<T extends bigint>(ptrs: T[]) {
  return Deno.UnsafePointer.of(new BigUint64Array(ptrs)) as Pointer<T>;
}

export function allocPtr<T>(): Pointer<T> {
  return ref(new BigUint64Array(1)) as Pointer<T>;
}

export function derefRef<T extends bigint>(ref: Pointer<T>): T {
  return new Deno.UnsafePointerView(ref).getBigUint64(0) as T;
}

export function readCString(message: bigint) {
  return new Deno.UnsafePointerView(message).getCString();
}

export function toCString(str: string) {
  return ref(
    Uint8Array.from(toCharCodes(str + "\0")),
  ) as Pointer<number>;
}

function toCharCodes(str: string): number[] {
  return [...str].map((_, i) => str.charCodeAt(i));
}
