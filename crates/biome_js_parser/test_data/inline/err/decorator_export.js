function foo() {
     @decorator
     export class Foo { }
     @first.field @second @(() => decorator)()
     export class Bar {}
}
