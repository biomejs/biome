/* should generate diagnostics */

// Name does not end in `Error`.
class Foo extends Error {
	constructor(message) {
		super(message);
		this.name = "Foo";
	}
}

// Name does not end in `Error` (non-`Error` built-in).
class Bar extends TypeError {
	constructor(message) {
		super(message);
		this.name = "Bar";
	}
}

// Constructor never assigns `this.name`.
class DatabaseError extends Error {
	constructor(message) {
		super(message);
	}
}

// `this.name` assigned dynamically.
class ApiError extends Error {
	constructor(message) {
		super(message);
		this.name = this.constructor.name;
	}
}

// `this.name` string literal doesn't match the class name.
class CacheError extends Error {
	constructor(message) {
		super(message);
		this.name = "WrongName";
	}
}
