function f() {
	"use strict"
	switch (x) {
		case 0: {
			function foo() {}
			break;
		}
		default: {
			function foo() {}
			break;
		}
	}
}

class C {
	method() {
		switch (x) {
			case 0: {
				function foo() {}
				break;
			}
			default: {
				function foo() {}
				break;
			}
		}
	}
}