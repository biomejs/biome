/* should generate diagnostics */

declare const extracted: Extract<Promise<void> | number, Promise<void>>;
extracted;

declare function wrap<T>(value: T): T extends string ? Promise<T> : T;
wrap("a");
