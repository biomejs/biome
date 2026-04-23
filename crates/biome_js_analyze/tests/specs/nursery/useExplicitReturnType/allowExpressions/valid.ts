/* should not generate diagnostics */

// Callbacks
fn(() => {});
fn(function () {});

// Array of function expressions
[function () {}, () => {}];

// Parenthesized function expression
(function () {});

// IIFE
(() => {})();

// Export default with return type
export default (): void => {};
