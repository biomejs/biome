/* should not generate diagnostics */

(() => { (eval as any)('var x = 1'); })();
(() => { eval!('var x = 1'); })();
(<T,>() => { run<T>(); })();
(function <T>() { run<T>(); })();
(() => { run(); })<string>();
(function (this: Window) { run(); })();
