class C {
    constructor(readonly a = b, readonly b = 0) {}
}

const member = E.A;
enum E {
    A = B,
    B,
}

namespace Ns {
	const c = new Class();
}
class Class {}

x;
import x = require("file");

function run1() {
	class A { @dec m() {} }
	const dec = (target: any, key: string) => {};
}
