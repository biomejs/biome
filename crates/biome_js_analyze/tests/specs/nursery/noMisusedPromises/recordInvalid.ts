/* should generate diagnostics */

// Record with async function values used in void-return context
const handlers: Record<string, (() => Promise<void>) | undefined> = {};
[1, 2, 3].forEach(async (n) => {
	await handlers[n.toString()]?.();
});

// Record with Promise values used directly in conditionals
const cache: Record<string, Promise<string>> = {};
if (cache["key"]) {
	console.log("cached");
}

while (cache["other"]) {
	break;
}

const val = cache["key"] ? "yes" : "no";
