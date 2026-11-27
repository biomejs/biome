/* should generate diagnostics */
class MissingNameError extends Error {}

class WrongNamePropertyError extends Error {
	name = "BadError";
}

class StaticNameError extends Error {
	static name = "StaticNameError";
}

class ComputedNameError extends Error {
	["name"] = "ComputedNameError";
}

class TemplateSubstitutionError extends Error {
	name = `${"TemplateSubstitutionError"}`;
}

class fooError extends Error {
	constructor(message, options) {
		super(message, options);
		this.name = "fooError";
	}
}

class Foo extends Error {
	constructor(message, options) {
		super(message, options);
		this.name = "Foo";
	}
}

class fooerror extends Error {
	name = "fooerror";
}

class NoSuperError extends Error {
	constructor() {}
}

class AssignedMessageError extends Error {
	constructor() {
		super();
		this.message = "My custom message";
	}
}

class EmptyConstructorError extends Error {
	constructor() {
		super();
	}
}

class ConstructorNameError extends Error {
	constructor() {
		super("My awesome Foo Error");
		this.name = this.constructor.name;
	}
}

class DuplicateMessageError extends Error {
	constructor(message) {
		super(message);
		this.message = message;
		this.name = "DuplicateMessageError";
	}
}

class MessageBeforeSuperError extends Error {
	constructor(message) {
		if (message instanceof Error) {
			this.originalError = message;
			message = message.message;
		}

		super();
		this.name = "MessageBeforeSuperError";
		this.message = message;
	}
}

class WrongLiteralNameError extends Http.ProtocolError {
	constructor() {
		super();
		this.name = "foo";
	}
}

class WrongTemplateNameError extends Error {
	constructor(message, options) {
		super(message, options);
		this.name = `foo`;
	}
}

module.exports = class ModuleError extends TypeError {
	constructor() {
		super();
		this.name = "foo";
	}
};

exports.fooError = class ExportError extends Error {
	constructor(error, options) {
		super(error, options);
		this.name = "ExportError";
	}
};

exports["fooError"] = class ComputedExportError extends Error {
	constructor(error, options) {
		super(error, options);
		this.name = "ComputedExportError";
	}
};

class MissingOptionsError extends Error {
	constructor(message) {
		super(message);
		this.name = "MissingOptionsError";
	}
}

class DefaultMessageError extends Error {
	constructor(message = "Default message") {
		super(message);
		this.name = "DefaultMessageError";
	}
}

class CauseParameterError extends Error {
	constructor(cause) {
		super("Fixed message", cause);
		this.name = "CauseParameterError";
	}
}

class InlineDetailsError extends Error {
	constructor(details) {
		super("Fixed message", { details });
		this.name = "InlineDetailsError";
	}
}

class InlineExtraOptionError extends Error {
	constructor(cause) {
		super("Fixed message", { cause, code: 1 });
		this.name = "InlineExtraOptionError";
	}
}

class ExtraArgumentError extends Error {
	constructor(cause) {
		super("Fixed message", { cause }, cause);
		this.name = "ExtraArgumentError";
	}
}

class ComputedMessageError extends Error {
	constructor(cause) {
		super(getMessage(), { cause });
		this.name = "ComputedMessageError";
	}
}

class UnrelatedCauseError extends Error {
	constructor(message, options) {
		super("Fixed message", { cause: new TypeError("other") });
		this.name = "UnrelatedCauseError";
	}
}

class DroppedOptionsError extends Error {
	constructor(message) {
		super(message, { cause });
		this.name = "DroppedOptionsError";
	}
}

class NotForwardedOptionsError extends Error {
	constructor(message, options) {
		super(message);
		this.name = "NotForwardedOptionsError";
	}
}

class OptionsObjectAsCauseError extends Error {
	constructor(message, options) {
		super(message, { cause: options });
		this.name = "OptionsObjectAsCauseError";
	}
}

class MissingMessageError extends Error {
	constructor(message, options) {
		super();
		this.name = "MissingMessageError";
	}
}

class UndefinedMessageError extends Error {
	constructor(message, options) {
		super(undefined, options);
		this.name = "UndefinedMessageError";
	}
}

class OptionsAsMessageError extends Error {
	constructor(options) {
		super(options);
		this.name = "OptionsAsMessageError";
	}
}

class FixedMessageOptionsError extends Error {
	constructor(options) {
		super("Fixed message");
		this.name = "FixedMessageOptionsError";
	}
}

class DetailsParameterError extends Error {
	constructor(message, details) {
		super(message);
		this.details = details;
		this.name = "DetailsParameterError";
	}
}

class GlobalThisBaseError extends globalThis.Error {
	constructor(message, details) {
		super(message, details);
		this.name = "GlobalThisBaseError";
	}
}

class OptsError extends TypeError {
	constructor(message, opts) {
		super(message, opts);
		this.name = "OptsError";
	}
}

class OptionsLastError extends Error {
	constructor(status, message, options) {
		super(message);
		this.name = "OptionsLastError";
	}
}

class GetterWithMessageError extends Error {
	#message;
	constructor(message) {
		super(message);
		this.#message = message;
		this.name = "GetterWithMessageError";
	}
	get message() {
		return this.#message;
	}
}

class GetterWithoutOptionsError extends Error {
	#message;
	constructor(message) {
		super();
		this.#message = message;
		this.name = "GetterWithoutOptionsError";
	}
	get message() {
		return this.#message;
	}
}

class GetterAssignmentError extends Error {
	constructor(message) {
		super();
		this.message = message;
		this.name = "GetterAssignmentError";
	}
	get message() {
		return "Custom message";
	}
}

class SetterWithMessageError extends Error {
	constructor(message) {
		super(message);
		this.name = "SetterWithMessageError";
	}
	set message(message) {
		this._message = message;
	}
}

const NamedExpressionError = class NamedExpression extends Error {
	name = "NamedExpression";
};

export default class DefaultExportedError extends TypeError {
	constructor() {
		super();
		this.name = "foo";
	}
}

class RestOptionsError extends Error {
	constructor(message, ...options) {
		super(message, ...options);
		this.name = "RestOptionsError";
	}
}
