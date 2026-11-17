// should generate diagnostics

declare const flag: boolean;

function maybeName() {
	if (flag) return { name: "a" };
}

// A path reaches the end of `maybeName`, so the call can give `undefined`.
const named = maybeName() || { name: "b" };
