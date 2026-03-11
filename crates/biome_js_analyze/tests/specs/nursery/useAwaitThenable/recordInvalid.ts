/* should generate diagnostics */

// Awaiting a Record value that is a string (not a Promise)
const config: Record<string, string> = {};
await config["key"];

// Awaiting a Record function value that returns a non-Promise
const getters: Record<string, () => number> = {};
await getters["count"]();

// Awaiting a Record function value with optional chaining that returns a non-Promise
// The ?.() call on (() => number) | undefined resolves to number | undefined,
// which is not a Promise and should be flagged.
const computers: Record<string, (() => number) | undefined> = {};
await computers["sum"]?.();
