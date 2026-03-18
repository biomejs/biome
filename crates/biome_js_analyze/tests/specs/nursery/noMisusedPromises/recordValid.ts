/* should not generate diagnostics */

// Record with non-Promise function values used in conditionals
const actions: Record<string, (() => void) | undefined> = {};
if (actions["save"]) {
	actions["save"]();
}

// Record with boolean values used in conditionals
const flags: Record<string, boolean> = {};
if (flags["enabled"]) {
	console.log("enabled");
}

// Record with Promise values properly awaited before conditional
const fetchers: Record<string, (() => Promise<boolean>) | undefined> = {};
async function checkFetcher() {
	const result = await fetchers["check"]?.();
	if (result) {
		console.log("ok");
	}
}
