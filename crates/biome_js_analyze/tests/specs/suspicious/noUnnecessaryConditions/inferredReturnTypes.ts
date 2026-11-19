// should generate diagnostics

declare const flag: boolean;
declare function fail(message: string): never;

function maybeBox() {
	if (flag) return { size: 1 };
}

function alwaysBox() {
	if (flag) return { size: 1 };
	fail("missing");
}

function nestedMember() {
	const helper = { fallback() { return null; } };
	return { size: 1 };
}

// A path reaches the end of `maybeBox`, so the call can give `undefined`.
function reachableEnd() {
	const box = maybeBox();
	return box?.size;
}

// No path reaches the end of `alwaysBox`.
function unreachableEnd() {
	const box = alwaysBox();
	return box?.size;
}

// The `return` of the nested method returns from the method.
function nestedReturn() {
	const box = nestedMember();
	return box?.size;
}

// The write assigns the inferred return type of `alwaysBox`.
function assignedCall() {
	let box: { size: number } | undefined = undefined;
	box = alwaysBox();
	return box?.size;
}
