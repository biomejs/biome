// Strict equality - positive check
const a = x !== undefined && x !== null ? x : y;
const b = x !== null && x !== undefined ? x : y;

// Strict equality - negative check
const c = x === undefined || x === null ? y : x;
const d = x === null || x === undefined ? y : x;

// Loose equality
const e = x != null ? x : y;
const f = x == null ? y : x;

// Single checks
const g = x !== null ? x : y;
const h = x !== undefined ? x : y;

// Complex expressions
const i = obj?.prop !== null && obj?.prop !== undefined ? obj?.prop : fallback;

// Nested
function test(val) {
  return val !== undefined && val !== null ? val : 'default';
}

// In array
const arr = [
  x !== null ? x : 'default'
];
