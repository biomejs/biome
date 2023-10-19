class A { #foo; #foo; }
class A { #foo() {}; #foo; }
class A { #foo; get #foo() {} }
class A { #foo; set #foo(value) {} }
class A { get #foo() {} get #foo() {} }
class A { get #foo() {}; #foo; }
class A { set #foo(value) {} #foo; }
class A { get #foo() {} set #foo(value) {} #foo; }
class A { #foo; #foo; #bar; #bar; }
class A { #foo; get #foo() {} get #foo() {} }
class A { foo; foo; #bar; #bar; }
class A { #foo; static #foo; }
class A { #foo; accessor #foo; }
