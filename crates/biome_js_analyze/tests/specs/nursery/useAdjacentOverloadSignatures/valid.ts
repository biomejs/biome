declare namespace Foo {
  export function foo_declare(s: string): void;
  export function foo_declare(n: number): void;
  export function foo_declare(sn: string | number): void;
  export function bar_declare(): void;
}

type Foo = {
  foo_type(s: string): void;
  foo_type(n: number): void;
  foo_type(sn: string | number): void;
  bar_type(): void;
};

interface Foo {
  foo_interface(s: string): void;
  foo_interface(n: number): void;
  foo_interface(sn: string | number): void;
  bar_interface(): void;
}

class A {
  fooA(s: string): void;
  fooA(n: number): void;
  fooA(sn: string | number): void {}
  barA(): void {}
}

class B {
  fooB(s: string): void;
  fooB(n: number): void;
  fooB(sn: string | number): void {};
  barB(): void {};
}

class C {
  fooC(s: string): void;
  fooC(s: string): void;
  fooC(s: string): void;
  fooC(sn: string | number): void {};
  barC(): void {};
  barC(): void {};
  barC(): void {};
}

export function bar(): void;
export function foo(s: string): void;
export function foo(n: number): void;
export function foo(sn: string | number): void;

function f() {
  interface Inner {
    innterInterfaceA(x: boolean): boolean
    intterInterfaceA(x: number): number
    innterInterfaceB()
  }
}

function f (): {
  functionB(),
  functionA(x: boolean): boolean,
  functionA(x: number): number,
} {
}

// Issue https://github.com/biomejs/biome/issues/3309#issuecomment-2208870371
class C {
  #f(): void {}
  g(): void {}
  f(): void {}
}