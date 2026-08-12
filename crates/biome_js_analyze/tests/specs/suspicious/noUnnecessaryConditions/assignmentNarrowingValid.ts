/* should not generate diagnostics */

function conditionalWrite(x: string | undefined, flag: boolean) {
	x = "on";
	if (flag) {
		x = undefined;
	}
	// `x` may be `undefined` again, so this condition is necessary.
	if (x) {
		x;
	}
}

function closureWrite(x: string | undefined) {
	x = "on";
	const reset = () => {
		x = undefined;
	};
	reset();
	// The closure may have written `x`, so this condition is necessary.
	if (x) {
		x;
	}
}

function compoundWrite(x: number | undefined) {
	x = 1;
	x += 1;
	// Compound assignments are not narrowing sources.
	if (x) {
		x;
	}
}
