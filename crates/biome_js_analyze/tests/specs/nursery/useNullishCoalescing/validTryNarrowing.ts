// should not generate diagnostics

function inTry(value: string | null) {
	try {
		if (value === null) return;
		const label = value || "none";
	} catch {
	}
}

function afterFinally(value: string | null) {
	try {
		if (value === null) return;
	} finally {
	}
	const label = value || "none";
}
