/* should generate diagnostics */
Foo.prototype.bar = function() {};
(function(){}());
f(function(){});
var a = new Date(function() {});
var test = function(d, e, f) {};
new function() {};
var foo = function() {};
var obj = { foo: function() {} };
(foo = function(){});
({foo = function(){}} = {});
[foo = function(){}] = [];
function fn(foo = function(){}) {}
class C { foo = function() {}; }
class D { [foo] = function() {}; }
class E { #foo = function() {}; }
var asyncFn = async function() {};
var gen = function*() {};
var asyncGen = async function*() {};
var gen2 = function *() {};
