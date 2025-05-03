export interface Cache<T> {
  has(key: string): boolean;
  get(key: string): T | undefined;
  set(key: string, val: T): this;
}
