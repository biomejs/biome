/* should generate diagnostics */
abstract class OuterError extends Error {
	static create() {
		class InnerError extends Error {
			constructor() {
				super();
			}
		}
		return new InnerError();
	}
}

abstract class HttpError extends Error {
	public abstract readonly status: number;
}

export class NotFoundError extends HttpError {
	public readonly status = 404;
}

export class TypeOnlyNameError extends Error {
	"name": SomeType;
	constructor(message) {
		super(message);
	}
}

exports[("fooError" as const)] = class ExportError extends Error {
	constructor(message, options) {
		super(message, options);
		this.name = "ExportError";
	}
};

class TypedMissingOptionsError extends Error {
	constructor(message: string) {
		super(message);
		this.name = "TypedMissingOptionsError";
	}
}

class AssertedBaseError extends (Error as ErrorConstructor) {
	constructor(message: string, details: unknown) {
		super(message);
		this.name = "AssertedBaseError";
	}
}
