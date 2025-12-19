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

// Bracket notation with allowed variables
const nodeEnvBracket = process.env["NODE_ENV"];
const ciBracket = process.env["CI"];
const viteBracket = import.meta.env["VITE_API_KEY"];

// Bun.env with allowed patterns
const bunNodeEnv = Bun.env.NODE_ENV;
const bunCi = Bun.env.CI;
const bunVite = Bun.env.VITE_APP_KEY;
const bunNodeEnvBracket = Bun.env["NODE_ENV"];

// Local process variable (not global) - should not be flagged
function test() {
    const process = { env: { LOCAL_VAR: 'test' } };
    return process.env.LOCAL_VAR;
}

// Local Bun variable (not global) - should not be flagged
function testBun() {
    const Bun = { env: { LOCAL_VAR: 'test' } };
    return Bun.env.LOCAL_VAR;
}

// Deno.env.get with allowed patterns
const denoNodeEnv = Deno.env.get("NODE_ENV");
const denoCi = Deno.env.get("CI");
const denoVite = Deno.env.get("VITE_APP_KEY");

// Local Deno variable (not global) - should not be flagged
function testDeno() {
    const Deno = { env: { get: () => 'test' } };
    return Deno.env.get("LOCAL_VAR");
}
