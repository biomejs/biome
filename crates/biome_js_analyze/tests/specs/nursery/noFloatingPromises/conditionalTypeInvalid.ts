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

type Payload<T> = T extends { data: infer P } | { error: infer P } ? P : never;
declare const payload: Payload<{ data: string; error: Promise<void> }>;
payload;

type PromiseIfOne<T> = T extends 1 ? Promise<void> : void;
declare const spelledOne: PromiseIfOne<1.0>;
spelledOne;

declare const fromUnknown: unknown extends {} | null | undefined ? Promise<void> : void;
fromUnknown;

declare const fromOptional: [undefined] extends [string?] ? Promise<void> : void;
fromOptional;

type Self<T> = T extends Promise<void> | (infer U) ? U : never;
declare const self: Self<Promise<void>>;
self;

type PromiseUnlessNever<T> = [T] extends [never] ? void : Promise<void>;
declare const narrowed: PromiseUnlessNever<string & "a">;
narrowed;
