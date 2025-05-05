/* should not generate diagnostics */
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