/* should not generate diagnostics */
new Object(x);
Object(x);
Object(...args);
new globalThis.Object();
globalThis.Object();
window.Object();
Object.create(null);
const obj = {};
const createObject = (Object) => new Object();

function shadowed() {
	var Object;
	new Object;
}

{
	class Object {}
	new Object();
}
