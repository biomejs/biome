class A { foo() {} foo() {} }
!class A { foo() {} foo() {} };
class A { foo() {} foo() {} foo() {} }
class A { static foo() {} static foo() {} }
class A { foo() {} get foo() {} }
class A { set foo(value) {} foo() {} }
class A { foo; foo; }
class A { 'foo'() {} 'foo'() {} }
class A { foo() {} 'foo'() {} }
class A { static constructor() {} static 'constructor'() {} }
class A { foo; accessor foo; }
class A { get foo () {} accessor foo; }
class A { set foo (value) {} accessor foo; }
class A { foo() {} foo() {} bar() {} bar() {} }
class A { get foo() {} get foo() {} }
class A { foo() {} "foo"() {} }

// class A { #foo; #foo; } This is invalid syntax, parser should throw an error
