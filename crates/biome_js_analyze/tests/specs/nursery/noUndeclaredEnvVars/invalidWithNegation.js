/* should generate diagnostics - negation patterns exclude vars from wildcard */

// ACME_SECRET is excluded via "!ACME_SECRET" negation
const acmeSecret = process.env.ACME_SECRET;

// ACME_TOKEN is excluded via "!ACME_*" wildcard negation
const acmeToken = process.env.ACME_TOKEN;
const acmeOther = process.env.ACME_OTHER;

// INTERNAL_TOKEN is excluded via "!INTERNAL_TOKEN" negation
const internalToken = process.env.INTERNAL_TOKEN;

// These should still be valid (not excluded by negation)
// MY_VAR matches the * wildcard
const myVar = process.env.MY_VAR;
// PUBLIC_URL matches the * wildcard
const publicUrl = process.env.PUBLIC_URL;
