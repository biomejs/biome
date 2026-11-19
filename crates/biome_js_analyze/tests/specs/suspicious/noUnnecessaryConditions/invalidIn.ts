// should generate diagnostics
// `in` keeps the union members that declare the property, so a read of
// another property takes its type from those members, as in TypeScript.

interface Loaded {
	data: { size: number };
}

interface Failed {
	data: null;
	error: string;
}

type Result = Loaded | Failed;

function present(result: Result) {
	if ("error" in result) {
		if (result.data) {
			return 1;
		}
	}
}

function absent(result: Result) {
	if (!("error" in result)) {
		return result.data?.size;
	}
}

function operand(result: Result) {
	return !("error" in result) && result.data?.size;
}

function constantKey(result: Result) {
	const key = "error";
	if (!(key in result)) {
		return result.data?.size;
	}
}
