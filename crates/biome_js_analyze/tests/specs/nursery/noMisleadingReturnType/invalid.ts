function stringWiden(b: boolean): string { if (b) return "loading"; return "idle"; }
function numberWiden(b: boolean): number { if (b) return 200; return 404; }
function bigintWiden(b: boolean): bigint { if (b) return 1n; return 2n; }

const asConstArrow = (): string => "main" as const;
const asConstParen = (): string => ("north" as const);
const asConstNumber = (): number => 42 as const;
const asConstBlock = (): string => { return "only" as const; };

const arrowMulti = (): string => { if (Math.random() > 0.5) return "yes"; return "no"; };

const fnExprMulti = function(b: boolean): string { if (b) return "dark"; return "light"; };

function nestedPrune(): string {
    function inner(): number { return 42; }
    if (Math.random() > 0.5) return "a";
    return "b";
}

async function asyncMulti(b: boolean): Promise<string> { if (b) return "a"; return "b"; }

function unionSubset(b: boolean): "a" | "b" | "c" { if (b) return "a"; return "b"; }

function objectAsConst(): { a: string; b: string } {
    return { a: "x", b: "y" } as const;
}

async function asyncAsConst(): Promise<string> { return "hello" as const; }

function unwrap<T>(x: T): T | null { return x; }

function asConstVar(): string {
    const result = "hello" as const;
    return result;
}

function recordWider(): Record<string, string> {
    return { a: "x", b: "y" } as const;
}

function nullable(flag: boolean): string | null { if (flag) return "yes"; return null; }

const satisfiesConst = (): string => ("idle" as const) satisfies string;

function angleBracketConst(): string { return <const>"hello"; }

function tupleWiden(): [string, number] { return ["hello", 42] as const; }

function ternaryWiden(b: boolean): string { return b ? "a" : "b"; }

function pickWider(): Pick<{name: string, age: number}, "name"> { return {name: "hello"} as const; }

function omitWider(): Omit<{name: string, age: number}, "age"> { return {name: "hello"} as const; }

function readonlyWider(): Readonly<{name: string}> { return {name: "hello"} as const; }

class StatusClass { getStatus(b: boolean): string { if (b) return "loading"; return "idle"; } }

const modeObj = { getMode(b: boolean): string { if (b) return "dark"; return "light"; } };

class CodeClass { getCode(b: boolean): number { if (b) return 200; return 404; } }

const codeObj = { getCode(b: boolean): number { if (b) return 200; return 404; } };

class GetterClass { get code(): number { if (Math.random() > 0.5) return 200; return 404; } }

const getterObj = { get code(): number { if (Math.random() > 0.5) return 200; return 404; } };

class AsyncMethod { async getStatus(b: boolean): Promise<string> { if (b) return "loading"; return "idle"; } }

function partialBooleanUnion(b: boolean): boolean | null { if (b) return true; return null; }

const ternaryBoolean = (b: boolean): boolean | "skip" => b ? true : false;

function dateObject(): object { return new Date(); }
function mapObject(): object { return new Map(); }
function setObject(): object { return new Set(); }
function weakMapObject(): object { return new WeakMap(); }
function errorObject(): object { return new Error(); }

function objectWider(): object { return { retry: true }; }

class Foo { x = 1; }
function objectFromClass(): object { return new Foo(); }

function variantObject(b: boolean): object { if (b) return { a: 1 }; return { b: 2 }; }

declare const apiResponse: { a: number };
function resolvedObject(): object { return apiResponse; }

function castNarrowLiteral(): object { return {} as { a: number }; }
type NarrowShape = { a: number };
function castNarrowAlias(): object { return {} as NarrowShape; }
class MyLocalClass { y = 1; }
function castLocalClass(): object { return new MyLocalClass() as MyLocalClass; }
interface MyLocalInterface { z: string; }
function castLocalInterface(): object { return { z: "x" } as MyLocalInterface; }

type NarrowIntersection = { a: number } & { b: string };
function castNarrowIntersection(): object { return {} as NarrowIntersection; }
function inlineNarrowIntersection(): object {
    return {} as ({ a: number } & { b: string });
}

async function asyncObjectWider(): Promise<object> { return { a: 1 }; }

class ObjectMethodClass { m(): object { return { a: 1 }; } }

const objectMethodObj = { m(): object { return { a: 1 }; } };

function asConstObjectAnnotation(): object { return { a: 1 } as const; }

function arrayReturn(): object { return [1, 2, 3]; }
function arrayEmpty(): object { return []; }
function regexpReturn(): object { return /foo/; }
function fnExprReturn(): object { return (): void => {}; }
function fnDeclReturn(): object { function bar() {} return bar; }

function satisfiesNarrow(): object { return { a: 1 } satisfies { a: number }; }
function satisfiesObjectStillNarrow(): object { return { a: 1 } satisfies object; }
function angleNarrow(): object { return <{ a: number }>{ a: 1 }; }
function identifierBoundNarrow(): object { const x = { a: 1 }; return x; }

function mixedEmptyAndNonEmpty(b: boolean): object { if (b) return {}; return { a: 1 }; }
function spreadReturn(x: { a: number }): object { return { ...x }; }

type ObjectAndNarrowShape = object & { a: number };
function intersectionNarrowMember(): object { return {} as object & { a: number }; }
function intersectionNarrowAlias(): object { return {} as ObjectAndNarrowShape; }

interface NarrowInterface { a: number; }
function narrowInterfaceCast(): object { return {} as NarrowInterface; }

function ternaryObjectBranch(b: boolean): object { return b ? {} : { a: 1 }; }

class InheritedBase { y = 1; }
class ClassWithInheritedMembers extends InheritedBase {}
function inheritedClassInstance(): object { return new ClassWithInheritedMembers(); }

function objectPropertyAsConst(): { a: string } {
    return { a: "x" as const };
}

function parenthesizedObjectPropertyAsConst(): { a: string } {
    return { a: ("x" as const) };
}

function objectPropertyNegativeNumberAsConst(): { a: number } {
    return { a: -1 as const };
}

function objectPropertyTupleAsConst(): { a: [number, number] } {
    return { a: [1, 2] as const };
}

function objectPropertyAsConstWithSibling(): { a: string; b: number } {
    return { a: "x" as const, b: 1 };
}

function nestedObjectPropertyAsConst(): { outer: { a: string } } {
    return { outer: { a: "x" as const } };
}

function objectWithConstPropertyFromIdentifier(): { a: string } {
    const result = { a: "x" as const };
    return result;
}

function singleWithNull(): string | null { return "hello"; }
function singleWithExtraPrimitive(): string | number { return "hello"; }
function singleBoolWithNull(): boolean | null { return true; }
function singleWithExtraLiteral(): string | 0 { return "hello"; }
function threeVariantsSingleReturn(): string | number | null { return "hello"; }
function partialReduce(b: boolean): string | number | null { if (b) return "a"; return 1; }
function multiLiteralNarrow(b: boolean): string | null { if (b) return "a"; return "b"; }
function exactMatchDropNull(b: boolean): "a" | "b" | null { if (b) return "a"; return "b"; }
function asConstUnionAnnotation(): string | null { return "hello" as const; }

async function asyncUnionNull(): Promise<string | null> { return "hello"; }

const arrowUnionNull = (): string | null => "hello";

class UnionClass { getValue(): string | null { return "hello"; } }

const unionObj = { getValue(): string | null { return "hello"; } };

class UnionGetter { get value(): string | null { return "hello"; } }

function nestedUnion(): string | (number | null) { return "hello"; }

type SimpleAlias = string | null;
function simpleAliasUnion(): SimpleAlias { return "hello"; }

function throwBranchUnion(b: boolean): string | null {
    if (b) throw new Error("fail");
    return "hello";
}

async function asyncUnionBothReturns(b: boolean): Promise<string | null> {
    if (b) return "hello";
    return null;
}

function partialAbsorbUnion(b: boolean): "a" | "b" | string | null { if (b) return "a"; return null; }

function crossPrimitiveUnion(): "a" | string | 1 { return "a"; }
