function foo() {
     @decorator abstract class A {}
     @first.field @second @(() => decorator)()
     abstract class Bar {}
}
