interface Baz extends Foo {}

interface Foo {}

interface Foo extends Array<number> {}

interface Foo extends Array<number | {}> {}

interface Foo<T> extends Bar<T> {}

declare module FooBar {
  export interface Bar extends Baz {}
}
