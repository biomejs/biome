/* should not generate diagnostics - env vars declared in passThroughEnv */

// Task-level env from turbo.json tasks.build.env
const buildVar = process.env.BUILD_VAR;

// Task-level passThroughEnv from turbo.json tasks.build.passThroughEnv
const acmeToken = process.env.ACME_AUTH_TOKEN;
const dbUrl = process.env.DATABASE_URL;

// Wildcard patterns in passThroughEnv (ACME_CACHE_* matches any ACME_CACHE_ prefixed var)
const cacheHost = process.env.ACME_CACHE_HOST;
const cachePort = process.env.ACME_CACHE_PORT;
const cacheCreds = process.env.ACME_CACHE_CREDS;

// Also works with import.meta.env
const acmeTokenMeta = import.meta.env.ACME_AUTH_TOKEN;
const cacheHostMeta = import.meta.env.ACME_CACHE_HOST;
