/* should not generate diagnostics */
'use ' + 'strict';
var a = 1;
function foo() {
	'x' + 'y';
	'use strict';
	return this;
}
