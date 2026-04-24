/* should generate diagnostics */

// IIFE returning a function without return type
const foo1 = (function () {
  return () => {
    return 1;
  };
})();

// Non-IIFE function still requires return type even with allowIIFEs
let foo2 = function () {
  return 'foo';
};

// Chained IIFE - inner arrow without return type
let foo3 = (() => () => {})()();
