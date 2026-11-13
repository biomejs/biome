// should generate diagnostics
// A property path narrows like a variable: after the test, `options.timer`
// and `config.retry` cannot be nullish at the optional chains, as in
// TypeScript.

interface Timer {
	cancel(): void;
}

function stop(options: { timer: Timer | null }) {
	if (options.timer !== null) {
		options.timer?.cancel();
	}
}

function delay(config: { retry: { delay: number } | undefined }) {
	if (!config.retry) {
		return 0;
	}
	return config.retry?.delay;
}
