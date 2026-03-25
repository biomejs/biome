/* should generate diagnostics */

// Direct calls
setTimeout("alert('Hi!');", 100);
setInterval("alert('Hi!');", 100);
setImmediate("alert('Hi!');");

// Global object access
window.setTimeout("count = 5", 10);
window.setInterval("foo = bar", 10);
globalThis.setTimeout("alert('Hi!');", 100);
// Biome doesn't support `global` for global namespacing
// global.setInterval("alert('Hi!');", 100);

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

// Biome doesn't support `global` for global namespacing
// global["setTimeout"]("alert('Hi!');", 100);

// Optional chaining on member access
window?.setTimeout("alert('Hi!');", 100);
window?.setInterval("alert('Hi!');", 100);
globalThis?.setTimeout("alert('Hi!');", 100);

// Optional chaining on computed member
window?.["setTimeout"]("alert('Hi!');", 100);
globalThis?.["setInterval"]("alert('Hi!');", 100);

// Parenthesized string arguments
setTimeout(("alert('Hi!');"), 100);
setTimeout((("alert('Hi!');")), 100);
setTimeout(("a" + "b"), 100);
setTimeout((`alert('Hi!');`), 100);
