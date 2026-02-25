// should generate diagnostics for ternary nullish checks

// === Simple strict inequality with null ===
// a !== null ? a : b  ->  a ?? b
const v1 = a !== null ? a : 'default';

// a !== undefined ? a : b  ->  a ?? b
const v2 = a !== undefined ? a : 'default';

// === Simple strict equality with null (inverted) ===
// a === null ? b : a  ->  a ?? b
const v3 = a === null ? 'default' : a;

// a === undefined ? b : a  ->  a ?? b
const v4 = a === undefined ? 'default' : a;

// === Loose equality ===
// a != null ? a : b  ->  a ?? b (covers both null and undefined)
const v5 = a != null ? a : 'default';

// a != undefined ? a : b  ->  a ?? b
const v6 = a != undefined ? a : 'default';

// a == null ? b : a  ->  a ?? b (inverted, loose)
const v7 = a == null ? 'default' : a;

// a == undefined ? b : a  ->  a ?? b
const v8 = a == undefined ? 'default' : a;

// === Null on the left side ===
// null !== a ? a : b  ->  a ?? b
const v9 = null !== a ? a : 'default';

// null === a ? b : a  ->  a ?? b
const v10 = null === a ? 'default' : a;

// undefined !== a ? a : b  ->  a ?? b
const v11 = undefined !== a ? a : 'default';

// undefined === a ? b : a  ->  a ?? b
const v12 = undefined === a ? 'default' : a;

// === Compound checks ===
// a === null || a === undefined ? b : a  ->  a ?? b
const v13 = a === null || a === undefined ? 'default' : a;

// a !== null && a !== undefined ? a : b  ->  a ?? b
const v14 = a !== null && a !== undefined ? a : 'default';

// === Member expressions ===
const v15 = obj.prop !== null ? obj.prop : 'default';
const v16 = obj.prop === undefined ? 'default' : obj.prop;

// === Computed member expressions ===
const v17 = arr[0] !== null ? arr[0] : 'default';

// === Parenthesized test subject ===
const v18 = (a) !== null ? a : 'default';

// === Nested in assignment ===
let result;
result = a !== null ? a : 'fallback';

// === In variable declaration ===
const v19 = foo() !== undefined ? foo() : 'default';

// === Compound with undefined on left of || ===
const v20 = a === undefined || a === null ? 'default' : a;

// === Compound with reversed null/undefined order ===
const v21 = a !== undefined && a !== null ? a : 'default';
