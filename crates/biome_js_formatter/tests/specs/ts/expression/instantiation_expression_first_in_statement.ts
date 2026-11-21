// Without the parentheses, these start a declaration and don't parse.
(class {})<string>;
(function () {})<string>;
(class Foo {})<string>, foo;
(function f() {})<string> ?? bar;
