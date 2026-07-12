/* should not generate diagnostics */

// Well-formed custom error: name ends in `Error` and `this.name` matches.
class FooError extends Error {
	constructor(message) {
		super(message);
		this.name = "FooError";
	}
}

// Non-`Error` built-in, correctly named and labeled.
class BarError extends TypeError {
	constructor(message) {
		super(message);
		this.name = "BarError";
	}
}

// Labeled via an instance class field instead of the constructor.
class FieldError extends Error {
	name = "FieldError";
}

// Does not extend a built-in error — out of scope.
class NotAnError extends SomeBase {
	constructor() {
		super();
	}
}

// No `extends` clause — out of scope.
class Plain {
	constructor() {}
}
