// A read in a closure created after a test sees the test, as in TypeScript,
// so each statement below is a promise that nothing handles.

function tested(value: Promise<void> | null) {
	if (value !== null) {
		return () => {
			value;
		};
	}
}

function assigned(value: Promise<void> | null) {
	value ??= Promise.resolve();
	return () => {
		value;
	};
}
