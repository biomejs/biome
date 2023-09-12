class Foo {
	// one decorator without a newline
	@decorator.method(value) property;
	@decorator.method(value) method() {}
	@decorator.method(value) async method() {}
	@decorator.method(value) *method() {}
	@decorator.method(value) get getter() {}
	@decorator.method(value) set setter(val) {}

	// two decorators without a newline
	@decorator.method(value) @decorator.method(value) property;
	@decorator.method(value) @decorator.method(value) method() {}
	@decorator.method(value) @decorator.method(value) async method() {}
	@decorator.method(value) @decorator.method(value) *method() {}
	@decorator.method(value) @decorator.method(value) get getter() {}
	@decorator.method(value) @decorator.method(value) set setter(val) {}

	// one decorator with a newline
	@decorator.method(value)
	property;
	@decorator.method(value)
	method() {}
	@decorator.method(value)
	async method() {}
	@decorator.method(value)
	*method() {}
	@decorator.method(value)
	get getter() {}
	@decorator.method(value)
	set setter(val) {}

	// two decorators without a newline
	@decorator.method(value)
	@decorator.method(value) property;
	@decorator.method(value)
	@decorator.method(value) method() {}
	@decorator.method(value)
	@decorator.method(value) async method() {}
	@decorator.method(value)
	@decorator.method(value) *method() {}
	@decorator.method(value)
	@decorator.method(value) get getter() {}
	@decorator.method(value)
	@decorator.method(value) set setter(val) {}
}


class Foo {
	// one decorator without a newline
	/*before*/ @decorator.method(value) /*after*/ property;
	/*before*/ @decorator.method(value) /*after*/ method() {}
	/*before*/ @decorator.method(value) /*after*/ async method() {}
	/*before*/ @decorator.method(value) /*after*/ *method() {}
	/*before*/ @decorator.method(value) /*after*/ get getter() {}
	/*before*/ @decorator.method(value) /*after*/ set setter(val) {}

	// two decorators without a newline
	/*before*/ @decorator.method(value) /*middle*/ @decorator.method(value) /*after*/ property;
	/*before*/ @decorator.method(value) /*middle*/ @decorator.method(value) /*after*/ method() {}
	/*before*/ @decorator.method(value) /*middle*/ @decorator.method(value) /*after*/ async method() {}
	/*before*/ @decorator.method(value) /*middle*/ @decorator.method(value) /*after*/ *method() {}
	/*before*/ @decorator.method(value) /*middle*/ @decorator.method(value) /*after*/ get getter() {}
	/*before*/ @decorator.method(value) /*middle*/ @decorator.method(value) /*after*/ set setter(val) {}

	// one decorator with a newline
	@decorator.method(value) /*before*/
		/*after*/ property;
	@decorator.method(value) /*before*/
	/*after*/ method() {}
	@decorator.method(value) /*before*/
	/*after*/ async method() {}
	@decorator.method(value) /*before*/
	/*after*/ *method() {}
	@decorator.method(value) /*before*/
	/*after*/ get getter() {}
	@decorator.method(value) /*before*/
	/*after*/ set setter(val) {}

	// two decorators without a newline
	@decorator.method(value) /*before*/
	/*middle*/ @decorator.method(value) /*after*/ property;
	@decorator.method(value) /*before*/
	/*middle*/ @decorator.method(value) /*after*/ method() {}
	@decorator.method(value) /*before*/
	/*middle*/ @decorator.method(value) /*after*/ async method() {}
	@decorator.method(value) /*before*/
	/*middle*/ @decorator.method(value) /*after*/ *method() {}
	@decorator.method(value) /*before*/
	/*middle*/ @decorator.method(value) /*after*/ get getter() {}
	@decorator.method(value) /*before*/
	/*middle*/ @decorator.method(value) /*after*/ set setter(val) {}
}
