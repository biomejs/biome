function bar() {
     @decorator
     let a;
     @decorator @decorator2
     function Foo() { }
}
