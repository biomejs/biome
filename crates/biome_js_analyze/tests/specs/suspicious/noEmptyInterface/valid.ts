interface A extends B { prop: number; }

// valid because extending multiple interfaces
// can be used instead of a union type
interface Baz extends Foo, Bar {}

// See https://github.com/biomejs/biome/issues/959
declare module "external" {
  export interface App extends Services {}
  export interface Empty {}

  global {
    export interface App extends Services {}
  }
}

// Ignore all interfaces that extends a type
interface Baz extends Foo {}

interface Foo extends Array<number> {}

interface Foo extends Array<number | {}> {}

interface Foo<T> extends Bar<T> {}

declare module FooBar {
  export interface Bar extends Baz {}
}

namespace Ns {
  export interface Bar extends Baz {}
}
