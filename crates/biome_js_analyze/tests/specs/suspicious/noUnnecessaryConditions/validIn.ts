// should not generate diagnostics
// `in` keeps a member that declares the property as optional in both
// branches, and keeps the whole union when no member declares the property.

interface Done {
	data: { size: number };
	error?: undefined;
}

interface Failed {
	data: null;
	error: string;
}

type Result = Done | Failed;

function optional(result: Result) {
	if ("error" in result) {
		return result.data?.size;
	}
}

function undeclared(result: Result) {
	if ("other" in result) {
		return result.data?.size;
	}
}
