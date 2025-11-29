// Safe defaults - should report

// Object literals
const config = options || {};
const settings = prefs || { theme: 'dark' };

// Array literals
const items = list || [];
const values = data || [1, 2, 3];

// Non-empty string literals
const name = input || 'Anonymous';
const message = text || "default message";

// Template literals with content
const greeting = name || `Hello ${user}`;

// Function expressions
const handler = callback || function() {};
const process = fn || (() => {});

// Class expressions
const Component = CustomClass || class {};

// new expressions
const instance = existing || new Map();
const date = d || new Date();

// Member access (suggests object expected)
const value = config || defaults.value;
const item = cache || storage.getItem('key');

// Nested safe defaults
const nested = a || (b || {});

// In various contexts
let result;
result = foo || {};

function test(param) {
  return param || [];
}

const fn = (x) => x || 'default';

const obj = {
  prop: value || {}
};

const arr = [x || []];

const computed = (x || 'fallback').length;
