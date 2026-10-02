/* should generate diagnostics */
function foo(x: number): void { console.log(x); }
[1, 2, 3].map(x => foo(x));
[1, 2, 3].map(x => { return foo(x); });
[1, 2, 3].map(function(x) { return foo(x); });
[1, 2, 3].forEach(x => x * 2);
[1, 2, 3].forEach((x => 42));

type NoResult = void;
declare function alias(): NoResult;
declare function number(): number;
declare function mixed(): number | void;
declare function optional(): number | undefined;
[1].map(() => alias());
[1].forEach(() => number());
[1].map(() => mixed());
[1].forEach(() => mixed());
[1].forEach(() => optional());

declare function overloaded(value: string): void;
declare function overloaded(value: number): number;
[1].forEach(() => overloaded(1));
declare function identity<T>(value: T): T;
[1].forEach(() => identity(42));
[1].map(() => identity(foo(1)));

declare const numbers: number[];
declare const tuple: [number, string];
declare const map: Map<string, number>;
declare const set: Set<number>;
numbers.map(() => {});
tuple.map(() => {});
map.forEach(() => 42);
set.forEach(() => 42);

[1].map((): number => { if (condition) return 42; });
[1].map(() => { if (condition) return unknownCall(); });
