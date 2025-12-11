/* should generate diagnostics */

// Undeclared environment variables should trigger diagnostics
const myCustomVar = process.env.MY_CUSTOM_VAR;
const anotherVar = process.env.ANOTHER_UNDECLARED_VAR;
const acmeSecret = process.env.ACME_SECRET;

// Also invalid with import.meta.env
const importMetaVar = import.meta.env.CUSTOM_META_VAR;
