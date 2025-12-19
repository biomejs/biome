/* should not generate diagnostics - dynamic property access is not checked */

// Dynamic property access cannot be statically analyzed, so these are skipped
const key = "MY_VAR";
const dynamicVar = process.env[key];

// Computed property with string literal
const computedVar = process.env["ANOTHER_VAR"];

// Dynamic with template literal
const prefix = "ACME";
const templateVar = process.env[`${prefix}_TOKEN`];

// Dynamic with concatenation
const concatVar = process.env["ACME" + "_SECRET"];

// Dynamic access via variable
function getEnvVar(name) {
    return process.env[name];
}

// Also with import.meta.env
const dynamicMeta = import.meta.env[key];
const computedMeta = import.meta.env["CUSTOM_VAR"];

// These are valid because the rule only checks static member expressions
// (process.env.VAR_NAME), not computed/dynamic access (process.env[key])
