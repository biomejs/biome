/* should generate diagnostics - negation patterns exclude vars from wildcard */

// SECRET_KEY is excluded via "!SECRET_KEY" negation
const secretKey = process.env.SECRET_KEY;

// SECRET_TOKEN is excluded via "!SECRET_*" wildcard negation
const secretToken = process.env.SECRET_TOKEN;
const secretOther = process.env.SECRET_OTHER;

// PRIVATE_KEY is excluded via "!PRIVATE_KEY" negation
const privateKey = process.env.PRIVATE_KEY;

// These should still be valid (not excluded by negation)
// API_KEY matches the * wildcard
const apiKey = process.env.API_KEY;
// PUBLIC_URL matches the * wildcard
const publicUrl = process.env.PUBLIC_URL;
