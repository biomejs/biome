class Foo {
   @dec constructor() {}
   @dec [index: string]: { props: string }
}
class Quiz {
   @dec public constructor() {}
}
class Bar extends Foo {
   @dec
   constructor();
   constructor(a: String)
   constructor(a?: String) {}
}
declare class Baz {
  @dec method();
  @dec get foo();
  @dec set foo(a);
}
