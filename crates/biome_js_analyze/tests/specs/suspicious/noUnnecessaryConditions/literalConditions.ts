// should generate diagnostics

declare function fail(message: string): never;
declare const flag: boolean;

// Only the literal conditions are reported: `value?.length` can be reached
// where `value` is null, and reads in unreachable code keep their declared
// types.
function loopWithExit(value: string | null) {
	while (1) {
		if (value) break;
		if (flag) break;
	}
	return value?.length;
}

function branchNeverTaken(value: string | null) {
	if (value) {
		if (false) {
			return value?.length;
		}
	}
}

function afterFailure(value: string | null) {
	if (value) {
		fail("stop");
		return value?.length;
	}
}

function inferredFailure(value: string | null) {
	const stop = () => fail("stop");
	if (value === null) stop();
	return value?.length;
}
