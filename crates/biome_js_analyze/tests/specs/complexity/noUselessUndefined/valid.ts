// Tests adapted from https://github.com/sindresorhus/eslint-plugin-unicorn/blob/488d5e30d4df02634bab336a55436f4e7479ca21/test/no-useless-undefined.js.
/* should not generate diagnostics */
async function foo(bar: boolean): Promise<string | undefined> {
	await new Promise(() => {});
	if (bar) {
		return "";
	}
	return undefined;
}

function union(): number | undefined {
	return undefined;
}

function reversedUnion(): undefined | number {
	return undefined;
}

function voidUnion(): number | void {
	return undefined;
}

function anyValue(): any {
	return undefined;
}

function unknownValue(): unknown {
	return undefined;
}

function neverValue(): never {
	return undefined;
}

function stringValue(): string {
	return undefined;
}

async function promiseVoid(): Promise<void> {
	return undefined;
}

export default function (): number | undefined {
	return undefined;
}

const functionExpression = function (): number | undefined {
	return undefined;
};

const arrowFunction = (): number | undefined => {
	return undefined;
};

class Example {
	method(): number | undefined {
		return undefined;
	}

	get value(): number | undefined {
		return undefined;
	}
}

const object = {
	method(): number | undefined {
		return undefined;
	},

	get value(): number | undefined {
		return undefined;
	},
};

function outer(): void {
	function inner(): number | undefined {
		return undefined;
	}

	inner();
}
