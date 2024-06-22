declare namespace Foo {
  export function foo_declare(s: string): void;
  export function bar_declare(): void;
  export function foo_declare(n: number): void;
  export function foo_declare(sn: string | number): void;
}

type Foo = {
  foo_type(s: string): void;
  foo_type(n: number): void;
  bar_type(): void;
  foo_type(sn: string | number): void;
};

interface Foo {
  foo_interface(s: string): void;
  foo_interface(n: number): void;
  bar_interface(): void;
  foo_interface(sn: string | number): void;
}

class A {
  fooA(s: string): void;
  fooA(n: number): void;
  barA(): void {};
  fooA(sn: string | number): void {};
}

class B {
  fooB(s: string): void;
  barB(): void {};
  fooB(n: number): void;
  fooB(sn: string | number): void {};
}

class C {
  barC(): void {};
  fooC(s: string): void;
  fooC(s: string): void;
  barC(): void {};
  fooC(s: string): void;
  fooC(sn: string | number): void {};
  barC(): void {};
}

class D {
  fooD(s: string): void;
  fooD(s: string): void;
  barD(): void {};
  fooD(n: number): void;
  fooD(sn: string | number): void {};
  fooD(sn: string | number): void {};
}

export function foo(s: string): void;
export function foo(n: number): void;
export function bar(): void;
export function foo(sn: string | number): void;

function f() {
  interface Inner {
    interfaceA(x: boolean): boolean
    interfaceB()
    interfaceA(x: number): number
  }
}

function g() {
  type InnerType = {
    typeMethodA(x: boolean): boolean;
    typeMethodB(): void;
    typeMethodA(x: number): number;
  };
}

function h (): {
  functionA(x: boolean): boolean,
  functionB(),
  functionA(x: number): number,
} {
}
