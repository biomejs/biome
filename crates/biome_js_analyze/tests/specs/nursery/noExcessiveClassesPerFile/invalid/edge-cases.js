/* should generate diagnostics */
class Outer {
	method() {
		class Inner { }
	}
}

(function() {
	return class Foo { };
})();
