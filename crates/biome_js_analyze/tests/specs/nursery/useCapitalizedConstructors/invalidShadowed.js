/* should generate diagnostics */
function String(x) { return x; } String(42);
{ let Array = () => {}; Array(42); }
function foo(Number) { return Number(42); }
{ const obj = {}; obj.String(42); }
{ const window = {}; window.String(42); }
{ const globalThis = {}; globalThis.String(42); }
{ const Date = { UTC() {} }; Date.UTC(2000, 0); }
function bar(Date) { return Date.UTC(2000, 0); }
function baz(globalThis) { return globalThis.Date.UTC(2000, 0); }
