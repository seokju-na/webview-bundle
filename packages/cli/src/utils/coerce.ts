export function coerceArray<T>(val: T | T[] | [T, ...T[]]): T[] {
  return Array.isArray(val) ? val : [val];
}
