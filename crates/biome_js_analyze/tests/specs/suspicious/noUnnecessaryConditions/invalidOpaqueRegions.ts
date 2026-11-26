// should generate diagnostics
// A `switch`, `try`, or `for...of` statement in a function does not stop
// narrowing, and reads inside one keep the facts from before it, so `value` is
// a `string` at each optional chain, as in TypeScript.

declare const items: string[];

function switchElsewhere(value: string | null, kind: number) {
	switch (kind) {
		case 1:
			break;
	}
	if (value !== null) {
		return value?.length;
	}
}

function readInsideTry(value: string | null) {
	if (value === null) {
		return;
	}
	try {
		return value?.length;
	} catch {
		return 0;
	}
}

function readInsideForOf(value: string | null) {
	if (value === null) {
		return;
	}
	for (const item of items) {
		value?.length;
	}
}
