// should generate diagnostics
// An element access with a string or integer literal key reads the same
// property as a named access, so after each test the receiver of the optional
// chain cannot be nullish, as in TypeScript. A write through a literal key
// writes only the property it names, so `keepOther` keeps the narrowing of
// `options.timer`.

interface Timer {
	cancel(): void;
}

function stop(options: { timer: Timer | null }) {
	if (options["timer"]) {
		options.timer?.cancel();
	}
}

function contentLength(headers: Record<string, string | undefined>) {
	if (headers["content-type"] !== undefined) {
		return headers["content-type"]?.length;
	}
	return 0;
}

function first(pair: [Timer | null, number]) {
	if (pair[0]) {
		pair[0]?.cancel();
	}
}

function keepOther(options: { timer: Timer | null; next: Timer | null }) {
	if (options.timer !== null) {
		options["next"] = null;
		options.timer?.cancel();
	}
}
