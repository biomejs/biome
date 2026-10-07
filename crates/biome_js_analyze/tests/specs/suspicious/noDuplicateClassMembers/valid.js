/* should not generate diagnostics */
class A { foo() {} bar() {} }
class A { static foo() {} foo() {} }
class A { get foo() {} set foo(value) {} }
class A { static foo() {} get foo() {} set foo(value) {} }
class A { foo() { } } class B { foo() { } }
class A { [foo]() {} foo() {} }
class A { 'foo'() {} 'bar'() {} baz() {} }
class A { *'foo'() {} *'bar'() {} *baz() {} }
class A { get 'foo'() {} get 'bar'() {} get baz() {} }
class A { 1() {} 2() {} }
class A { ['foo']() {} ['bar']() {} }
class A { [`foo`]() {} [`bar`]() {} }
class A { [12]() {} [123]() {} }
class A { [1.0]() {} ['1.0']() {} }
class A { [0x1]() {} [`0x1`]() {} }
class A { [null]() {} ['']() {} }
class A { get ['foo']() {} set ['foo'](value) {} }
class A { ['foo']() {} static ['foo']() {} }
class A { ['constructor']() {} constructor() {} }
class A { constructor() {} get [`constructor`]() {} }
class A { ['foo' + '']() {} ['foo']() {} }
class A { [`foo${''}`]() {} [`foo`]() {} }
class A { [-1]() {} ['-1']() {} }
class A { [foo]() {} [foo]() {} }
class A { foo; static foo; }
class A { foo; #foo; }
class A { '#foo'; #foo; }
class A { [10]() {} [11]() {} }
class A { 0x10() {} 0x11() {} }
class A { [1n]() {} [2n]() {} }

