/* should generate diagnostics */
class Basic {
	constructor(public foo: string) {
		this.foo = foo;
	}
}

class NonNull {
	constructor(public foo?: string) {
		this.foo = foo!;
	}
}

class AsExpression {
	constructor(public foo?: string) {
		this.foo = foo as any;
	}
}

class TypeAssertion {
	constructor(public foo: string) {
		this.foo = <string>foo;
	}
}

class Satisfies {
	constructor(public foo: string) {
		this.foo = (foo satisfies string);
	}
}

class DefaultValue {
	constructor(public foo = "") {
		this.foo = foo;
	}
}

class ChangedAfter {
	constructor(public foo = "") {
		this.foo = foo;
		this.foo += "foo";
	}
}

class LogicalOr {
	constructor(public foo: string) {
		this.foo ||= foo;
	}
}

class Nullish {
	constructor(public foo: string) {
		this.foo ??= foo;
	}
}

class LogicalAnd {
	constructor(public foo: string) {
		this.foo &&= foo;
	}
}

class StringKey {
	constructor(private foo: string) {
		this["foo"] = foo;
	}
}

class TemplateKey {
	constructor(private foo: string) {
		this[`foo`] = foo;
	}
}

class OtherModifiers {
	constructor(protected readonly foo: string) {
		this.foo = foo;
	}
}

class NestedFunctionBefore {
	constructor(private foo: string) {
		function bar() {
			this.foo = foo;
		}
		this.foo = foo;
	}
}

class NestedArrowBefore {
	constructor(private foo: string) {
		this.bar = () => {
			this.foo = foo;
		};
		this.foo = foo;
	}
}

class NestedClassBefore {
	constructor(private foo: string) {
		class Bar {
			constructor(private baz: string) {}
		}
		this.foo = foo;
	}
}

class FieldWritesInFunction {
	constructor(private foo: string) {
		this.foo = foo;
	}
	bar = () => {
		this.foo = "foo";
	};
}

class StaticFieldWrite {
	constructor(private foo: string) {
		this.foo = foo;
	}
	static init = (this.foo = "foo");
}

class Block {
	constructor(private foo: string) {
		{
			this.foo = foo;
		}
	}
}

class Conditional {
	constructor(private foo: string, flag: boolean) {
		if (flag) {
			this.foo = foo;
		}
	}
}

class ArrowIife {
	constructor(private foo: string) {
		(() => {
			this.foo = foo;
		})();
	}
}

class ParameterChangedAfter {
	constructor(public name: string) {
		this.name = name;
		name = name.trim();
	}
}

class OtherParameterChanged {
	constructor(
		public name: string,
		public age: number,
	) {
		name = name.trim();
		this.name = name;
		this.age = age;
	}
}

class LocalVariableChanged {
	constructor(public name: string) {
		let local = name;
		local = local.trim();
		this.name = name;
	}
}

// Both assignments are reported.
class Repeated {
	constructor(public foo: string) {
		this.foo = foo;
		this.foo = foo;
	}
}

class InLoop {
	constructor(public foo: string, items: string[]) {
		for (const item of items) {
			this.foo = foo;
		}
	}
}

class ParameterChangedInLaterLoop {
	constructor(public foo: string) {
		this.foo = foo;
		for (let i = 0; i < 2; i++) {
			foo = foo.trim();
		}
	}
}

class OtherPropertyDeleted {
	constructor(public foo?: number, public bar?: number) {
		delete this.bar;
		this.foo = foo;
	}
}

// Both assignments are reported.
class BothBranches {
	constructor(public foo: string, flag: boolean) {
		if (flag) {
			this.foo = foo;
		} else {
			this.foo = foo;
		}
	}
}
