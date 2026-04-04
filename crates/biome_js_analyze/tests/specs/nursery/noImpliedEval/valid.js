/* should not generate diagnostics */

// Function argument (correct usage)
setTimeout(function() {
    alert('Hi!');
}, 100);

setInterval(() => {
    alert('Hi!');
}, 100);

// Shadowed by local variable
function foo(setTimeout) {
    setTimeout("alert('Hi!');", 100);
}

const bar = (setInterval) => {
    setInterval("alert('Hi!');", 100);
};

// Non-global object
const obj = { setTimeout: function() {} };
obj.setTimeout("alert('Hi!');", 100);

// Different function name
setTimeout(callback, 100);

// Template with substitution
const code = "alert('Hi!');";
setTimeout(`${code}`, 100);

// Non-string argument
setTimeout(123, 100);
setTimeout(null, 100);
setTimeout(undefined, 100);

// Function as first argument
setTimeout(function() {}, 100);

// Computed member on non-global object
const timer = { setTimeout: () => {} };
timer["setTimeout"]("code", 100);

// Unknown object (can't determine if global)
someObject["setTimeout"]("code", 100);
someObject.setInterval("code", 100);

// Call with .call/.apply (different signature)
setTimeout.call(null, function() {}, 100);
setTimeout.apply(null, [function() {}, 100]);

// Non-global member chain (window.foo is not global)
window.foo.setTimeout("code", 100);
globalThis.bar.setInterval("code", 100);

// Optional chaining with non-global object
obj?.setTimeout("code", 100);
obj?.["setTimeout"]("code", 100);

// Optional chaining with function argument
window?.setTimeout(() => {}, 100);
globalThis?.setInterval(function() {}, 100);

// Parenthesized function arguments
setTimeout((function() {}), 100);
setTimeout((() => {}), 100);
