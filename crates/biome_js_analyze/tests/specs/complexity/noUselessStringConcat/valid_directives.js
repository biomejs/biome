/* should not generate diagnostics */
'use ' + 'client';
export default function foo() {
	'use ' + 'strict';
	return this;
}
function bar() {
	'use asm';
	'use ' + 'strict';
}
const baz = () => {
	'use ' + 'strict';
};
