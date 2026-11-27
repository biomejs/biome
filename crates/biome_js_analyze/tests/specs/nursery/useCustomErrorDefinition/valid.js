/* should not generate diagnostics */
class Foo {}
class Bar extends Baz {}
class Qux extends Quux() {}
const Anonymous = class {};
const AnonymousError = class extends Error {
	constructor(message) {
		super(message);
	}
};

class FooError extends Error {
	constructor(message, options) {
		super(message, options);
		this.name = "FooError";
	}
}

class DetailsError extends Error {
	constructor(message, options) {
		super(message, options);
		this.details = options?.details;
		this.name = "DetailsError";
	}
}

class FileError extends Error {
	constructor(filePath, options) {
		super(`File not found: ${filePath}`, options);
		this.name = "FileError";
	}
}

class RestError extends Error {
	constructor(...args) {
		super(...args);
		this.name = "RestError";
	}
}

class OptionsOnlyError extends Error {
	constructor(options) {
		super("Fixed message", options);
		this.name = "OptionsOnlyError";
	}
}

class DefaultOptionsError extends Error {
	constructor(message, options = {}) {
		super(message, options);
		this.name = "DefaultOptionsError";
	}
}

class InlineCauseError extends Error {
	constructor(cause) {
		super("The request timed out", { cause });
		this.name = "InlineCauseError";
	}
}

class InlineCauseValueError extends Error {
	constructor(error) {
		super("The request timed out", { cause: error });
		this.name = "InlineCauseValueError";
	}
}

class InlineCauseStringKeyError extends Error {
	constructor(error) {
		super(`The request timed out`, { "cause": error });
		this.name = "InlineCauseStringKeyError";
	}
}

class InlineCauseExtraParameterError extends Error {
	constructor(code, cause) {
		super("Fixed message", { cause });
		this.name = "InlineCauseExtraParameterError";
	}
}

class ForwardedCauseError extends Error {
	name = "ForwardedCauseError";
	constructor(message, options) {
		super(message, { cause: options.cause });
	}
}

class OptionalForwardedCauseError extends Error {
	name = "OptionalForwardedCauseError";
	constructor(message, options) {
		super(message, { cause: options?.cause });
	}
}

class ApiError extends Error {
	constructor(status, message, options) {
		super(message, options);
		this.name = "ApiError";
		this.status = status;
	}
}

class OptionsFirstError extends Error {
	constructor(options, status, message) {
		super(message, options);
		this.name = "OptionsFirstError";
	}
}

class NoArgumentsError extends Error {
	constructor() {
		super("My custom error");
		this.name = "NoArgumentsError";
	}
}

class EmptyTypeError extends TypeError {
	constructor() {
		super();
		this.name = "EmptyTypeError";
	}
}

class PropertyError extends Error {
	name = "PropertyError";
}

class TemplatePropertyError extends Error {
	name = `TemplatePropertyError`;
}

class TemplateAssignmentError extends Error {
	constructor(message, options) {
		super(message, options);
		this.name = `TemplateAssignmentError`;
	}
}

// Custom error classes can take different arguments.
class ProtocolFooError extends Http.ProtocolError {
	constructor(message) {
		super(message);
		this.name = "ProtocolFooError";
	}
}

class UserNotFoundError extends GraphQLError {
	constructor(userId) {
		super(`User "${userId}" does not exist`, { extensions: { code: "NOT_FOUND" } });
		this.name = "UserNotFoundError";
	}
}

class CustomBaseError extends Custom.Error {
	constructor(message) {
		super(message);
		this.name = "CustomBaseError";
	}
}

class TimeoutError extends FetchError {
	constructor(cause) {
		super("The request timed out", { cause });
		this.name = "TimeoutError";
	}
}

class ManyError extends AggregateError {
	constructor(errors, message, options) {
		super(errors, message, options);
		this.name = "ManyError";
	}
}

class GlobalManyError extends globalThis.AggregateError {
	constructor(errors, message, options) {
		super(errors, message, options);
		this.name = "GlobalManyError";
	}
}

class DisposeError extends SuppressedError {
	constructor(error, suppressed, message) {
		super(error, suppressed, message);
		this.name = "DisposeError";
	}
}

// `Error` is a local variable here, not the built-in error.
function shadowed(Error) {
	class ShadowedError extends Error {
		constructor(message) {
			super(message);
			this.name = "ShadowedError";
		}
	}
}

class OtherMessageError extends Error {
	constructor() {
		super();
		this.name = "OtherMessageError";
		someThingNotThis.message = "My custom message";
	}
}

class AppendMessageError extends Error {
	constructor(message, options) {
		super(message, options);
		this.message += message;
		this.name = "AppendMessageError";
	}
}

class GetterError extends Error {
	#message;

	constructor(message, options) {
		super(undefined, options);
		this.#message = message;
		this.name = "GetterError";
	}

	get message() {
		return this.#message;
	}
}

class AccessorsError extends Error {
	#message;

	constructor(message, options) {
		super(undefined, options);
		this.message = message;
		this.name = "AccessorsError";
	}

	get message() {
		return this.#message;
	}

	set message(message) {
		this.#message = message;
	}
}

class SetterError extends Error {
	constructor(message, options) {
		super(undefined, options);
		this.message = message;
		this.name = "SetterError";
	}

	set message(message) {
		this._message = message;
	}
}

export class ExportedError extends TypeError {
	constructor() {
		super();
		this.name = "ExportedError";
	}
}

export default class DefaultExportedError extends TypeError {
	constructor() {
		super();
		this.name = "DefaultExportedError";
	}
}

module.exports = class ModuleError extends TypeError {
	constructor() {
		super();
		this.name = "ModuleError";
	}
};

exports.CommonJsError = class CommonJsError extends TypeError {
	constructor() {
		super();
		this.name = "CommonJsError";
	}
};

exports["ComputedExportError"] = class ComputedExportError extends Error {
	constructor(message, options) {
		super(message, options);
		this.name = "ComputedExportError";
	}
};

const exportName = "DynamicExportError";
exports[exportName] = class DynamicExportError extends Error {
	constructor(message, options) {
		super(message, options);
		this.name = "DynamicExportError";
	}
};

exports.anonymousError = class extends Error {
	constructor(error) {
		super(error);
	}
};

exports.whatever = class Whatever {};
