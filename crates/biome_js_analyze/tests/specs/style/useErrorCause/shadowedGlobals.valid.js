// should not generate diagnostics

// A local `Error` is a custom class, so its options may come last.
class Error extends BaseError {
	constructor(message, code, options) {
		super(message, options);
		this.code = code;
	}
}

try {
	doSomething();
} catch (err) {
	throw new Error("Something went wrong", "E_CODE", { cause: err });
}

function withLocalGlobalThis(globalThis) {
	try {
		doSomething();
	} catch (err) {
		throw new globalThis.Error("Something went wrong", "E_CODE", { cause: err });
	}
}
