interface A extends B {
  prop: number;
}

// valid because extending multiple interfaces
// can be used instead of a union type
interface Baz extends Foo, Bar {}

// See https://github.com/biomejs/biome/issues/959
declare module "external" {
  export interface App extends Services {}

  global {
    export interface App extends Services {}
  }
}