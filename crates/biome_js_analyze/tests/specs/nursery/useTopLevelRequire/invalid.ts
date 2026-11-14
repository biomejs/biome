/* should generate diagnostics */
function load() {
	return require('a') as typeof import('a');
}
namespace Foo {
	const b = require('b');
}
