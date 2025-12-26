/* should not generate diagnostics */

const a = Object.getPrototypeOf(obj);
const b = {
	__proto__: a,
	val: 12
}
Object.setPrototypeOf(obj, b);
