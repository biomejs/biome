/* should generate diagnostics - env vars NOT declared in turbo.json */

// These vars are NOT in the turbo.json, so they should produce diagnostics
const undeclaredVar = process.env.UNDECLARED_VAR;
const anotherUndeclared = process.env.ANOTHER_UNDECLARED;

// This doesn't match the FOO_* pattern (it's BAR_*)
const barTest = process.env.BAR_TEST;

// Also invalid with import.meta.env
const undeclaredMeta = import.meta.env.UNDECLARED_META;
