/* should generate diagnostics */

declare const extracted: Extract<Promise<void> | number, Promise<void>>;
extracted;

declare function wrap<T>(value: T): T extends string ? Promise<T> : T;
wrap("a");

type PromiseIfString<T> = T extends string ? Promise<T> : T;
type AliasOfAlias<T> = PromiseIfString<T>;
declare const aliased: AliasOfAlias<string>;
aliased;

declare function whenTrue<T>(value: T): T extends true ? Promise<void> : void;
declare const flag: boolean;
whenTrue(flag);
