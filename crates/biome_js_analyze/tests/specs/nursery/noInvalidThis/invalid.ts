/* should generate diagnostics */
interface SomeType {
	prop: string;
}
function foo() {
	this.prop;
}
function withParameters(a: string, b = this) {}
const typed: Foo = function () {
	this.a = 0;
};
