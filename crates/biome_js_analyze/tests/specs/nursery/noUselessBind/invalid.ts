/* should generate diagnostics */
var a = (function () { foo(); } as Callback).bind(bar);
var b = (function () { foo(); } satisfies Callback).bind(bar);
var c = (function () { foo(); })!.bind(bar);
var d = function (this: Foo) { foo(); }.bind(bar);
