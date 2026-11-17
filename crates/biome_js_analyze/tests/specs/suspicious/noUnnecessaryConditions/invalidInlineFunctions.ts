// should generate diagnostics
// A function called where it is defined and a class static block run in the
// flow of the code around them, so their writes and the paths that they end
// narrow the code after them, as in TypeScript.

function assignedInPlace(value: string | null) {
	(() => {
		value = "a";
	})();
	return value ?? "b";
}

function throwsInPlace(value: string | null) {
	(function () {
		if (value === null) {
			throw new Error("missing");
		}
	})();
	return value ?? "b";
}

function assignedInStaticBlock(value: string | null) {
	class Config {
		static {
			value = "a";
		}
	}
	return value ?? "b";
}
