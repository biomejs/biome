// should not generate diagnostics
// A catch clause starts from the state before the `try` statement, a finally
// block sees every way into it, including exceptions, and the code after the
// statement sees the end of a catch clause that completes normally, as in
// TypeScript.

function inCatch(value: string | null) {
	try {
		if (value === null) throw new Error("missing");
	} catch {
		return value?.length;
	}
}

function inFinally(value: string | null) {
	try {
		if (value === null) return;
	} finally {
		value?.length;
	}
}

function afterCatch(value: string | null) {
	try {
		if (value === null) return;
	} catch {
	}
	return value?.length;
}
