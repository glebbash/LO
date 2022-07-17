export function stringToBytes(str: string): Uint8Array {
  return new Uint8Array(str.length).fill(0).map((_, i) => str.charCodeAt(i));
}

export function bytesToString(bytes: Uint8Array): string {
  return String.fromCharCode(...bytes);
}
