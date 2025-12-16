/* should not generate diagnostics - env vars declared in turbo.jsonc */

// Global env vars from turbo.jsonc globalEnv
const acmeToken = process.env.JSONC_ACME_TOKEN;
const acmeAppId = process.env.JSONC_ACME_APP_ID;

// Task-level env vars from turbo.jsonc tasks.build.env
const buildVar = process.env.JSONC_BUILD_VAR;
const buildOutput = process.env.JSONC_BUILD_OUTPUT;

// Wildcard patterns (JSONC_FOO_* matches any JSONC_FOO_ prefixed var)
const fooBar = process.env.JSONC_FOO_BAR;
const fooBaz = process.env.JSONC_FOO_BAZ;
const fooSomethingElse = process.env.JSONC_FOO_SOMETHING_ELSE;

// Also works with import.meta.env
const acmeTokenMeta = import.meta.env.JSONC_ACME_TOKEN;
const fooBarMeta = import.meta.env.JSONC_FOO_BAR;

// Still allowed by defaults (not from turbo.jsonc)
const nodeEnv = process.env.NODE_ENV;
const ci = process.env.CI;
