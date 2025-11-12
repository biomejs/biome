// Basic || that should be ??
const value = x || 'default';

// Multiple || operators
const value2 = a || b || c;

// Assignment context
let result;
result = foo || 'fallback';

// Inside function
function test(param) {
  return param || 0;
}

// Arrow function
const fn = (x) => x || [];

// Object property
const obj = {
  prop: value || 'default'
};

// Array element
const arr = [x || y];

// Nested in expression
const computed = (x || 5) * 2;
