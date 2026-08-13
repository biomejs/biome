if (array.every(Boolean));

if (array.length > 0 && array.every(Boolean)) {}

if (array.length === 0 || anotherCheck() || array.every(Boolean));

if (array.length === 0 || array.some(Boolean));

const a = something();

// should not generate diagnostics
// Different base object: not the same array.
if (a.length === 0 || b.some(Boolean));