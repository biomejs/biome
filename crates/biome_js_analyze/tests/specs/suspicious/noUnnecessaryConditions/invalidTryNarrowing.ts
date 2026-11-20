// should generate diagnostics
// Conditions in a try block, a catch clause, and a finally block narrow as
// anywhere else, and the code after a `try` statement continues from the ends
// of its try block and its catch clause, as in TypeScript.

declare function check(): void;

function inTry(value: string | null) {
	try {
		if (value !== null) {
			return value?.length;
		}
	} finally {}
}

function afterCatchReturns(value: string | null) {
	try {
		if (value === null) return;
	} catch {
		return;
	}
	return value?.length;
}

function afterFinally(value: string | null) {
	try {
		if (value === null) return;
	} finally {
	}
	return value?.length;
}

function testInFinally(value: string | null) {
	try {
		check();
	} finally {
		if (value === null) throw new Error("missing");
	}
	return value?.length;
}

function catchVariable() {
	try {
		check();
	} catch (error) {
		if (error instanceof Error) {
			return error?.message;
		}
	}
}
