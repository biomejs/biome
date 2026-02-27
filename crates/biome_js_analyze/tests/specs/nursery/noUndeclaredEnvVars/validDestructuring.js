/* should not generate diagnostics - destructuring is not checked */

// Destructuring from process.env is not detected by this rule
// because it doesn't create a static member expression (process.env.VAR_NAME)
const { MY_VAR, ANOTHER_VAR, ACME_SECRET } = process.env;

// Destructuring with rename
const { DATABASE_URL: dbUrl, ACME_TOKEN: acmeToken } = process.env;

// Nested destructuring (edge case)
const { env: { CUSTOM_VAR } } = process;

// Destructuring with defaults
const { OPTIONAL_VAR = "default" } = process.env;

// Destructuring with rest
const { NODE_ENV, ...rest } = process.env;

// Also with import.meta.env
const { VITE_VAR, CUSTOM_META } = import.meta.env;

// Mixed destructuring and assignment
let someVar;
({ someVar } = process.env);

// Note: This is a known limitation of the rule - it only checks
// static member access (process.env.VAR_NAME), not destructuring patterns
