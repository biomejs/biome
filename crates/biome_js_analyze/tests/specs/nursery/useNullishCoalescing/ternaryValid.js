/* should not generate diagnostics */

// Not a nullish check - comparing with a non-nullish value
const v1 = a !== 0 ? a : 'default';
const v2 = a !== '' ? a : 'default';
const v3 = a !== false ? a : 'default';

// Subject in branch doesn't match test subject
const v4 = a !== null ? b : 'default';
const v5 = a === null ? 'default' : b;

// Non-equality operators
const v6 = a > null ? a : 'default';
const v7 = a < undefined ? a : 'default';

// Different subjects on both sides of compound check
const v8 = a === null || b === undefined ? 'default' : a;

// Compound with loose equality (not supported)
const v9 = a == null || a == undefined ? 'default' : a;

// Compound with mismatched equality direction
const v10 = a === null || a !== undefined ? 'default' : a;

// Compound with wrong logical operator
const v11 = a === null && a === undefined ? 'default' : a;
const v12 = a !== null || a !== undefined ? a : 'default';

// Compound checking same nullish kind twice
const v13 = a === null || a === null ? 'default' : a;
const v14 = a === undefined || a === undefined ? 'default' : a;

// Wrong branch matches - inequality but alternate matches (should be consequent)
const v15 = a !== null ? 'default' : a;

// Wrong branch matches - equality but consequent matches (should be alternate)
const v16 = a === null ? a : 'default';

// Already using ??
const v17 = a ?? 'default';

// Regular ternary with boolean check
const v18 = a ? b : c;

// Ternary with typeof check (not a nullish check)
const v19 = typeof a === 'undefined' ? 'default' : a;

// Comparing two variables (neither is null/undefined)
const v20 = a !== b ? a : 'default';
