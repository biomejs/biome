// js module
"use strict"; // Associated comment

function foo() {
	"use strict";
}

class C1 {
	// All code here is evaluated in strict mode
	test() {
		"use strict";
	}
}

const C2 = class {
	// All code here is evaluated in strict mode
	test() {
		"use strict";
	}
};
