/* should generate diagnostics */
class InConstructor {
	constructor(private foo: string) {
		class Bar {
			constructor(private foo: string) {
				this.foo = foo;
			}
		}
	}
}

class InField {
	constructor(private foo: string) {}
	init = class Bar {
		constructor(private foo: string) {
			this.foo = foo;
		}
	};
}
