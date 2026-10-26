// should not generate diagnostics

class ErrorWithCode extends Error {
	constructor(message, code, options) {
		super(message, options);
		this.code = code;
	}
}

class ErrorWithTrailingCode extends Error {
	constructor(message, options, code) {
		super(message, options);
		this.code = code;
	}
}

try {
	doSomething();
} catch (err) {
	throw new ErrorWithCode("Something went wrong", "E_CODE", { cause: err });
}

try {
	doSomething();
} catch (err) {
	throw new ErrorWithTrailingCode("Something went wrong", { cause: err }, "E_CODE");
}

try {
	doSomething();
} catch (cause) {
	throw new ErrorWithCode("Something went wrong", "E_CODE", { cause });
}

try {
	doSomething();
} catch (err) {
	throw new errors.ErrorWithCode("Something went wrong", "E_CODE", { cause: err });
}

try {
	doSomething();
} catch (err) {
	throw new AggregateError([err], "Something went wrong", { cause: err });
}

try {
	doSomething();
} catch (err) {
	throw new globalThis.Error("Something went wrong", { cause: err });
}
