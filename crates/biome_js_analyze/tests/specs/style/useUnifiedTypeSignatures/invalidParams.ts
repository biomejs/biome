// Destructuring doesn't count since they don't have names

function f(foo: string, {bar}: Record<"bar", string>): string;
function f(foo: number, {barrer}: Record<"barrer", string>): string;
function f(whatever: any): any {}

function f2(foo: string, {bar}: Record<"bar", string>): string;
function f2(foo: string, {barrer}: Record<"barrer", number>): string;
function f2(whatever: any): any {}

// neither does array spread
function g(foo: string, [a, b]: [number, string]): string;
function g(foo: number, [a, b]: [number, string]): string;
function g(whatever: any): any {}

// name mismatches are skipped, but subsequent ones still processed
function multi(foo: string, bar: string): void;
function multi(fork: number, bar: number): void;