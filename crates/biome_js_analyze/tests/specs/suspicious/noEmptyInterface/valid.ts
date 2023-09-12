interface A extends B {
  prop: number;
}

// valid because extending multiple interfaces
// can be used instead of a union type
interface Baz extends Foo, Bar {}
