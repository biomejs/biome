// ref: https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/eslint-plugin/tests/rules/no-invalid-void-type.test.ts
function takeVoid(thing: void) {}
const arrowGeneric = <T extends void>(arg: T) => {};
const arrowGeneric2 = <T extends void = void>(arg: T) => {};
function functionGeneric<T extends void>(arg: T) {}
function functionGeneric2<T extends void = void>(arg: T) {}
declare function functionDeclaration<T extends void>(arg: T): void;
declare function functionDeclaration2<T extends void = void>(arg: T): void;

declare function voidArray(args: void[]): void[];
let value = undefined as void;
let value = <void>undefined;
function takesThings(...things: void[]): void {}
type KeyofVoid = keyof void;

interface Interface {
  lambda: () => void;
  voidProp: void;
}

class ClassName {
  private readonly propName: void;
}
let letVoid: void;
type VoidType = void;
class OtherClassName {
  private propName: VoidType;
}

type UnionType = string | number | void;
type UnionType = string | ((number & any) | (string | void));
declare function test(): number | void;
declare function test<T extends number | void>(): T;
type IntersectionType = string & number & void;

type MappedType<T> = {
  [K in keyof T]: void;
};

type ConditionalType<T> = {
  [K in keyof T]: T[K] extends string ? void : string;
};
type ManyVoid = readonly void[];
function foo(arr: readonly void[]) {}
type invalidVoidUnion = void | Map<string, number>;
