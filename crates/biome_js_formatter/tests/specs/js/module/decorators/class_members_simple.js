class Foo {
	// one decorator without a newline
	@dec property;
	@dec method() {}
	@dec async method() {}
	@dec *method() {}
	@dec get getter() {}
	@dec set setter(val) {}

	// two decorators without a newline
	@dec @dec property;
	@dec @dec method() {}
	@dec @dec async method() {}
	@dec @dec *method() {}
	@dec @dec get getter() {}
	@dec @dec set setter(val) {}

	// one decorator with a newline
	@dec
	property;
	@dec
	method() {}
	@dec
	async method() {}
	@dec
	*method() {}
	@dec
	get getter() {}
	@dec
	set setter(val) {}

	// two decorators without a newline
	@dec
	@dec property;
	@dec
	@dec method() {}
	@dec
	@dec async method() {}
	@dec
	@dec *method() {}
	@dec
	@dec get getter() {}
	@dec
	@dec set setter(val) {}
}


class Foo {
	// one decorator without a newline
	/*before*/ @dec /*after*/ property;
	/*before*/ @dec /*after*/ method() {}
	/*before*/ @dec /*after*/ async method() {}
	/*before*/ @dec /*after*/ *method() {}
	/*before*/ @dec /*after*/ get getter() {}
	/*before*/ @dec /*after*/ set setter(val) {}

	// two decorators without a newline
	/*before*/ @dec /*middle*/ @dec /*after*/ property;
	/*before*/ @dec /*middle*/ @dec /*after*/ method() {}
	/*before*/ @dec /*middle*/ @dec /*after*/ async method() {}
	/*before*/ @dec /*middle*/ @dec /*after*/ *method() {}
	/*before*/ @dec /*middle*/ @dec /*after*/ get getter() {}
	/*before*/ @dec /*middle*/ @dec /*after*/ set setter(val) {}

	// one decorator with a newline
	@dec /*before*/
		/*after*/ property;
	@dec /*before*/
	/*after*/ method() {}
	@dec /*before*/
	/*after*/ async method() {}
	@dec /*before*/
	/*after*/ *method() {}
	@dec /*before*/
	/*after*/ get getter() {}
	@dec /*before*/
	/*after*/ set setter(val) {}

	// two decorators without a newline
	@dec /*before*/
	/*middle*/ @dec /*after*/ property;
	@dec /*before*/
	/*middle*/ @dec /*after*/ method() {}
	@dec /*before*/
	/*middle*/ @dec /*after*/ async method() {}
	@dec /*before*/
	/*middle*/ @dec /*after*/ *method() {}
	@dec /*before*/
	/*middle*/ @dec /*after*/ get getter() {}
	@dec /*before*/
	/*middle*/ @dec /*after*/ set setter(val) {}
}
