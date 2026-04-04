/* should not generate diagnostics - env vars declared in turbo.json */

// Global env vars from turbo.json globalEnv
const acmeToken = process.env.ACME_TOKEN;
const acmeAppId = process.env.ACME_APP_ID;

// Task-level env vars from turbo.json tasks.build.env
const buildVar = process.env.BUILD_VAR;
const buildOutput = process.env.BUILD_OUTPUT;

// Wildcard patterns (FOO_* matches any FOO_ prefixed var)
const fooBar = process.env.FOO_BAR;
const fooBaz = process.env.FOO_BAZ;
const fooSomethingElse = process.env.FOO_SOMETHING_ELSE;

// Also works with import.meta.env
const acmeTokenMeta = import.meta.env.ACME_TOKEN;
const fooBarMeta = import.meta.env.FOO_BAR;

// Still allowed by defaults (not from turbo.json)
const nodeEnv = process.env.NODE_ENV;
const ci = process.env.CI;
