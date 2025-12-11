/* should not generate diagnostics - env vars declared in passThroughEnv */

// Task-level env from turbo.json tasks.build.env
const buildVar = process.env.BUILD_VAR;

// Task-level passThroughEnv from turbo.json tasks.build.passThroughEnv
const awsKey = process.env.AWS_SECRET_KEY;
const dbUrl = process.env.DATABASE_URL;

// Wildcard patterns in passThroughEnv (REDIS_* matches any REDIS_ prefixed var)
const redisHost = process.env.REDIS_HOST;
const redisPort = process.env.REDIS_PORT;
const redisPassword = process.env.REDIS_PASSWORD;

// Also works with import.meta.env
const awsKeyMeta = import.meta.env.AWS_SECRET_KEY;
const redisHostMeta = import.meta.env.REDIS_HOST;
