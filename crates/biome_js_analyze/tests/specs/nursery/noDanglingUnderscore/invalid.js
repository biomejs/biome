/* should generate diagnostics */
var _foo = 1;
let foo_ = 1;
const __bar__ = 1;
var __proto__ = 1;
for (const _item of items) {}
for (let _i = 0; _i < 10; _i++) {}
function _init() {}
function init_() {}
foo._bar;
foo?._bar;
foo._bar();
foo.bar._baz;
this._count;
this.constructor._instances;
foo._bar = 1;
[foo._bar] = [1];
({ a: foo._bar } = {});
delete foo._bar;
class A extends B {
	constructor() {
		super._prop;
	}
}
