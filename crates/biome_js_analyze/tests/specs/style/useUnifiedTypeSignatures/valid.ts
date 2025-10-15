/* should not generate diagnostics */
function g(): void;
function g(a: number, b: number): void;
function g(a?: number, b?: number): void {}

function rest(...xs: number[]): void;
function rest(xs: number[], y: string): void;
function rest(...args: any[]) {}

class C {
  constructor();
  constructor(a: number, b: number);
  constructor(a?: number, b?: number) {}

  a(): void;
  a(a: number, b: number): void;
  a(a?: number, b?: number): void {}
}

declare class Example1 {
  privateMethod(a: number): void;
  #privateMethod(a: number, b?: string): void;
}

declare class Example2 {
  #privateMethod1(a: number): void;
  #privateMethod2(a: number, b?: string): void;
}

// No error for arity difference greater than 1.
interface I {
  a2(): void;
  a2(x: number, y: number): void;
}

// No error for different return types.
interface I {
  a4(): void;
  a4(x: number): number;
}

// No error if one takes a type parameter and the other doesn't.
interface I {
  a5<T>(x: T): T;
  a5(x: number): number;
}

// No error if one is a rest parameter and other isn't.
interface I {
  b2(x: string): void;
  b2(...x: number[]): void;
}

// No error if both are rest parameters. (https://github.com/Microsoft/TypeScript/issues/5077)
interface I {
  b3(...x: number[]): void;
  b3(...x: string[]): void;
}

// No error if one is optional and the other isn't.
interface I1 {
  c3(x: number): void;
  c3(x?: string): void;
}

// No error if they differ by 2 or more parameters.
interface I2 {
  d2(x: string, y: number): void;
  d2(x: number, y: string): void;
}

// No conflict between static/non-static members.
declare class D {
  static a();
  a(x: number);
}

// Allow signatures if the type is not equal.
interface I3 {
  f(x1: number): void;
  f(x1: boolean, x2?: number): void;
}

// Type parameters are not equal
function f1<T extends number>(x: T[]): void;
function f1<T extends string>(x: T): void;
function f1<T extends string>(x: unknown) {}

declare module 'hello' {
  function foo(n: number, s: string): number;
  function foo(n: string, s: number): number;
}

export interface Foo {
  bar(baz: string): number[];
  bar(): string[];
}

declare module 'foo' {
  export default function (foo: number): string[];
}

export default function (foo: string): string[];
export default function (foo: unknown): string[] {
    return [String(foo)];
}

// https://github.com/typescript-eslint/typescript-eslint/issues/740
function p(key: string): Promise<string | undefined>;
function p(key: string, defaultValue: string): Promise<string>;
function p(key: string, defaultValue?: string): Promise<string | undefined> {
  const obj: Record<string, string> = {};
  return obj[key] || defaultValue;
}

interface I {
  p<T>(x: T): Promise<T>;
  p(x: number): Promise<number>;
}

function rest(...xs: number[]): Promise<number[]>;
function rest(y: string): Promise<string>;
async function rest(...args: any[]): Promise<number[] | string> {
  return args;
}

declare class Foo1 {
  get bar();
  set bar(x: number);
}

interface Foo2 {
  get bar();
  set bar(x: number);
}

abstract class Foo3 {
  abstract get bar();
  abstract set bar(a: unknown);
}

interface I4 {
  a(x?: string): void;
  a(x: number): void;
}

// Cannot make "this" optional.
declare function f10(): void;
declare function f10(this: number): void;
