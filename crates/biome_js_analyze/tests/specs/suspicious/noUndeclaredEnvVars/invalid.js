/* should generate diagnostics */

// Undeclared environment variables should trigger diagnostics
const myCustomVar = process.env.MY_CUSTOM_VAR;
const anotherVar = process.env.ANOTHER_UNDECLARED_VAR;
const acmeSecret = process.env.ACME_SECRET;

// Also invalid with import.meta.env
const importMetaVar = import.meta.env.CUSTOM_META_VAR;

// Bracket notation with string literals should also be checked
const bracketVar = process.env["BRACKET_VAR"];
const bracketMeta = import.meta.env["BRACKET_META_VAR"];

// Bun.env should also be checked
const bunVar = Bun.env.BUN_CUSTOM_VAR;
const bunBracketVar = Bun.env["BUN_BRACKET_VAR"];

// Deno.env.get should also be checked
const denoVar = Deno.env.get("DENO_CUSTOM_VAR");
const denoVar2 = Deno.env.get("ANOTHER_DENO_VAR");
