// different name from implementation, but same between overloads
function f(foo: string): string;
function f(foo: number): string;
function f(param: any): string {return ""}

// Destructuring doesn't count since they don't have names
function f2(foo: string, {bar}: Record<"bar", string>): string;
function f2(foo: string, {barrer}: Record<"bar" | "barrer", string>): string;
function f2(whatever: any): string {return "foooarafv"}

// neither does array spread
function g(foo: string, [a, b]: [number, string]): string;
function g(foo: string, [a, b]: [string, string]): string;
function g(whatever: any): any {}
