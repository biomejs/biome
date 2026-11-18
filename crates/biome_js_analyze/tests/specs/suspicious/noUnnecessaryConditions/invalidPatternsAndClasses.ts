// should generate diagnostics
// Destructuring, class expressions, and logical assignments elsewhere in a
// function do not stop narrowing, so `value` is a `string` at each optional
// chain, as in TypeScript.

function destructuredParameter({ value }: { value: string | null }) {
	if (value !== null) {
		return value?.length;
	}
}

function destructuredConst(input: { value: string | null }) {
	const { value } = input;
	if (value !== null) {
		return value?.length;
	}
}

function classExpression(value: string | null) {
	const Holder = class {
		field = 1;
	};
	if (value !== null) {
		return value?.length;
	}
}

function logicalAssignment(value: string | null, count: number | undefined) {
	count ??= 0;
	if (value !== null) {
		return value?.length;
	}
}
