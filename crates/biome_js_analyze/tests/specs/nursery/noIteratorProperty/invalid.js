/* should generate diagnostics */
var a = test.__iterator__;
Foo.prototype.__iterator__ = function () {};
var b = test["__iterator__"];
var c = test['__iterator__'];
var d = test[`__iterator__`];
test[`__iterator__`] = function () {};
var e = test?.__iterator__;
var f = test?.["__iterator__"];
var g = test[("__iterator__")];
foo.bar.__iterator__.baz;
({ x: foo.__iterator__ } = obj);
[foo["__iterator__"]] = arr;
foo.__iterator__ += 1;
delete foo.__iterator__;
