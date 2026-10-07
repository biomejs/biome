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
class A { 10() {} 1e1() {} }
class A { ['foo']() {} ['foo']() {} }
class A { static ['foo']() {} static foo() {} }
class A { set 'foo'(value) {} set ['foo'](val) {} }
class A { ''() {} ['']() {} }
class A { [`foo`]() {} [`foo`]() {} }
class A { static get [`foo`]() {} static get ['foo']() {} }
class A { foo() {} [`foo`]() {} }
class A { get [`foo`]() {} 'foo'() {} }
class A { static 'foo'() {} static [`foo`]() {} }
class A { ['constructor']() {} ['constructor']() {} }
class A { static [`constructor`]() {} static constructor() {} }
class A { [123]() {} [123]() {} }
class A { [0x10]() {} 16() {} }
class A { [100]() {} [1e2]() {} }
class A { [123.00]() {} [`123`]() {} }
class A { static '65'() {} static [0o101]() {} }
class A { [123n]() {} 123() {} }
class A { [null]() {} 'null'() {} }
class A { 'constructor'() {} [`constructor`]() {} }
class A { 'constructor'() {} set ['constructor'](value) {} }

// class A { #foo; #foo; } This is invalid syntax, parser should throw an error
