// should generate diagnostics
// A conditional expression has the type of both of its branches, as in
// TypeScript, even when its test can never be falsy, so `label` can be `""`
// and its checks are needed. Only the test in `declaredObject` is reported:
// a value of type `object` is never falsy.

interface Entry {
	size: number;
}

function declaredObject(entry: object) {
	const label = entry ? "entry" : "";
	if (label) {
		return 1;
	}
}

function narrowedInterface(entry: Entry | undefined) {
	if (entry !== undefined) {
		const label = entry ? "entry" : "";
		if (label) {
			return 1;
		}
	}
}
