/* should not generate diagnostics */

function noAnnotation() { return "loading"; }
const noAnnotationArrow = () => "hello";

function greet(): string { return "hello"; }
function getCode(): number { return 200; }
function isReady(): boolean { return true; }
function getBigVal(): bigint { return 1n; }
const getTag = (): string => "main";
const getTagBlock = (): string => { return "main"; };
const getMode = function(): string { return "dark"; };

function returnVoid(): void { return; }
function returnAny(): any { return "data"; }
function returnUnknown(): unknown { return "data"; }
function returnNever(): never { throw new Error(); }

function noReturn(): void { console.log("done"); }

declare const someVar: string;
function returnVar(): string { return someVar; }

declare const s: string;
function echoString(): string { return s; }
declare const n: number;
function echoNumber(): number { return n; }

function exactMatch(): "north" | "south" { if (Math.random() > 0.5) return "north"; return "south"; }
function exactMatchThree(): "a" | "b" | "c" { if (Math.random() > 0.5) return "a"; if (Math.random() > 0.5) return "b"; return "c"; }
function exactMatchNumber(): 200 | 404 { if (Math.random() > 0.5) return 200; return 404; }

function* gen(): Generator<string> { yield "a"; }
function* genNumber(): Generator<number> { yield 42; }

function identity<T>(x: T): T { return x; }
function constrained<T extends string>(x: T): T { return x; }
function maybeNull<T>(x: T): T | null { if (Math.random() > 0.5) return x; return null; }

async function asyncSingle(): Promise<string> { return "hello"; }
const asyncArrow = async (): Promise<string> => "hello";
async function asyncNumber(): Promise<number> { return 200; }

async function asyncExact(): Promise<"a" | "b"> { if (Math.random() > 0.5) return "a"; return "b"; }

function findItem(arr: string[]): string | undefined { return arr[0]; }
function maybeReturn(flag: boolean): string | void { if (flag) return "yes"; }

declare const dynamicStr: string;
function mixedReturn(): string { if (Math.random() > 0.5) return "literal"; return dynamicStr; }

declare function fetchData(): string;
function callReturn(): string { return fetchData(); }

function computed(): number { return 1 + 2; }
function negated(flag: boolean): boolean { return !flag; }

function template(): string { return `hello`; }

function objectNoConst(): { a: string; b: string } {
    return { a: "x", b: "y" };
}

function asConstMatch(): "hello" { return "hello" as const; }

function bareReturn(): void { return; }

function throwOnly(): string { throw new Error("never"); }

function fib(n: number): number { if (n <= 1) return n; return fib(n - 1) + fib(n - 2); }

function outerWithNested(): number {
    function inner(): string { return "nested"; }
    return 42;
}

function withConstructor(): number {
    class Inner { constructor() { return Object.create(null); } }
    return 42;
}

function withNestedArrow(): number {
    const inner = (): string => "nested";
    return 42;
}

function sameComputed(): number { const x = 1 + 2; return x; }

function overloaded(x: "a"): "a";
function overloaded(x: "b"): "b";
function overloaded(x: "a" | "b"): string { if (x === "a") return "a"; return "b"; }

function complexAnnotation(): Record<string, string> { return { a: "b" }; }
function arrayAnnotation(): string[] { return ["a", "b"]; }

function booleanExhaustive(b: boolean): boolean { if (b) return true; return false; }

function anyReturn(): string { return JSON.parse("{}"); }

function tupleNoConst(): [boolean, string] { return [true, "hello"]; }

function ternaryExact(b: boolean): "a" | "b" { return b ? "a" : "b"; }

class Greeter { greet(): string { return "hello"; } }
const singleObj = { greet(): string { return "hello"; } };
class ExactClass { getStatus(b: boolean): "a" | "b" { if (b) return "a"; return "b"; } }
const exactObj = { getStatus(b: boolean): "a" | "b" { if (b) return "a"; return "b"; } };
class GetterValid { get name(): string { return "bar"; } }
const getterObj = { get name(): string { return "bar"; } };

class AsyncSingle { async fetch(): Promise<string> { return "data"; } }

// Generator methods should be skipped
class GeneratorClass {
    *items(): Generator<string> {
        yield "a";
        yield "b";
    }
}

// Overloaded class methods should be skipped
class OverloadedMethods {
    getStatus(b: true): "loading";
    getStatus(b: false): "idle";
    getStatus(b: boolean): string {
        if (b) return "loading";
        return "idle";
    }
}
