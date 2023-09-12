"use strict";
"use strict";

function test() {
	"use strict";
	function inner_a() {
		"use strict"; // redundant directive
	}
	function inner_b() {
		function inner_inner() {
			"use strict"; // additional redundant directive
		}
	}
}
