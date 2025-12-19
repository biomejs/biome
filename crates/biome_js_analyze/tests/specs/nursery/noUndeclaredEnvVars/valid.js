/* should not generate diagnostics */

// NODE_ENV is always allowed
const nodeEnv = process.env.NODE_ENV;

// CI variables are allowed
const ci = process.env.CI;

// Vercel variables are allowed
const vercelEnv = process.env.VERCEL_ENV;
const vercelUrl = process.env.VERCEL_URL;

// Framework-specific prefixes are allowed by default
const nextPublic = process.env.NEXT_PUBLIC_API_URL;
const viteVar = process.env.VITE_API_KEY;
const reactApp = process.env.REACT_APP_NAME;
const vueApp = process.env.VUE_APP_VERSION;
const gatsby = process.env.GATSBY_API_URL;
const expo = process.env.EXPO_PUBLIC_KEY;

// import.meta.env with allowed patterns
const viteEnv = import.meta.env.VITE_BASE_URL;
const nodeEnvMeta = import.meta.env.NODE_ENV;

// Local process variable (not global) - should not be flagged
function test() {
    const process = { env: { LOCAL_VAR: 'test' } };
    return process.env.LOCAL_VAR;
}
