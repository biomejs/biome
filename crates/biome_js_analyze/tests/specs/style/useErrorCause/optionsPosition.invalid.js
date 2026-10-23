class ErrorWithCode extends Error {
	constructor(message, code, options) {
		super(message, options);
		this.code = code;
	}
}

try {
	doSomething();
} catch (err) {
	throw new ErrorWithCode("Something went wrong", "E_CODE", { code: err.code });
}

try {
	doSomething();
} catch (err) {
	throw new Error("Something went wrong", "ignored", { cause: err });
}

try {
	doSomething();
} catch (err) {
	throw new AggregateError([err], { cause: err });
}

try {
	doSomething();
} catch (err) {
	const other = err;
	if (other) {
		const err = new Error("Unrelated");
		throw new ErrorWithCode("Something went wrong", "E_CODE", { cause: err });
	}
}
