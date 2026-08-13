if (array.every(Boolean));

if (array.length > 0 && array.every(Boolean)) {}

if (array.length === 0 || array.some(Boolean));

// should not generate diagnostics
// Different base object: not the same array.
if (a.length === 0 || b.some(Boolean));

// should not generate diagnostics
// Two identical calls may return different arrays.
if (foo().length === 0 || foo().every(Boolean));