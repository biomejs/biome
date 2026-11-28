/* should not generate diagnostics */

// The current value decides whether the right side is evaluated.
let cached = null;
export function load() {
	cached ??= fetch("/api").catch(() => {
		cached = null;
	});
}

let initialized = false;
initialized ||= init();

let enabled = true;
enabled &&= check();
