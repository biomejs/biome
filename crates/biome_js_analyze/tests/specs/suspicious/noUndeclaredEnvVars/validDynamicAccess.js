/* should not generate diagnostics - truly dynamic property access is not checked */

// Dynamic property access with variables cannot be statically analyzed, so these are skipped
const key = "MY_VAR";
const dynamicVar = process.env[key];

// Dynamic with template literal containing interpolation
const prefix = "ACME";
const templateVar = process.env[`${prefix}_TOKEN`];

// Dynamic with concatenation
const concatVar = process.env["ACME" + "_SECRET"];

// Dynamic access via variable
function getEnvVar(name) {
    return process.env[name];
}

// Also with import.meta.env - dynamic access
const dynamicMeta = import.meta.env[key];

// Also with Bun.env - dynamic access
const dynamicBun = Bun.env[key];

// Also with Deno.env.get - dynamic access
const dynamicDeno = Deno.env.get(key);

// These are valid because the rule cannot statically determine the key
// Note: String literal bracket access like process.env["VAR"] IS now checked
// Note: String literal in Deno.env.get("VAR") IS now checked
