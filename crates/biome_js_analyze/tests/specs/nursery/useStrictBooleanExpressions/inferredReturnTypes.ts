// should generate diagnostics

declare const flag: boolean;
declare const literal: 1 | undefined;

function maybeCount() {
	if (flag) return 1;
}

function maybeName(name: string) {
	if (flag) return name;
}

if (literal) {
}

// A path reaches the end of `maybeCount`, so the call can give `undefined`.
if (maybeCount()) {
}

if (maybeName("a")) {
}
