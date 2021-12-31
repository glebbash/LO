export function getStringValue(value: string): string {
  return value
    .slice(1, -1)
    .replace(/\\"/g, '"')
    .replace(/\\n/g, '\n')
    .replace(/\\\\/g, '\\');
}

export function getNumberValue(value: string): number {
  return parseFloat(value.replace(/_/g, ''));
}
