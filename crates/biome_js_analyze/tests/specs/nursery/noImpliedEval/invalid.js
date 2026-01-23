/* should generate diagnostics */

// Direct calls
setTimeout("alert('Hi!');", 100);
setInterval("alert('Hi!');", 100);
setImmediate("alert('Hi!');");

// Global object access
window.setTimeout("count = 5", 10);
window.setInterval("foo = bar", 10);
globalThis.setTimeout("alert('Hi!');", 100);
global.setInterval("alert('Hi!');", 100);

// Computed member
window["setTimeout"]("alert('Hi!');", 100);
globalThis["setInterval"]("foo = bar", 10);

// Chained global access
window.window.setTimeout("alert('Hi!');", 100);
globalThis.globalThis.setInterval("foo = bar", 10);

// Template literals (no substitution)
setTimeout(`alert('Hi!');`, 100);

// String concatenation
setTimeout("alert" + "('Hi!');", 100);
setInterval("foo" + "bar", 10);

// Sequence expression
(0, setTimeout)("alert('Hi!');", 100);
(0, window.setTimeout)("alert('Hi!');", 100);

// Computed member on global object
global["setTimeout"]("alert('Hi!');", 100);

// Nested sequence expression
(0, (0, setTimeout))("alert('Hi!');", 100);

// Optional chaining on member access
window?.setTimeout("alert('Hi!');", 100);
window?.setInterval("alert('Hi!');", 100);
globalThis?.setTimeout("alert('Hi!');", 100);

// Optional chaining on computed member
window?.["setTimeout"]("alert('Hi!');", 100);
globalThis?.["setInterval"]("alert('Hi!');", 100);

// Optional call on sequence expression
(0, setTimeout)?.("alert('Hi!');", 100);
(0, window.setTimeout)?.("alert('Hi!');", 100);

// Parenthesized optional call
((0, setTimeout))?.("alert('Hi!');", 100);
