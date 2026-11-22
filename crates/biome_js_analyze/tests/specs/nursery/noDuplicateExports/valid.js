/* should not generate diagnostics */
const a = 1;
export { a, a as b };

export const c = 1;
export { d } from "./d.js";
export { d as e } from "./e.js";

export * from "./f.js";
export * from "./g.js";
export * as h from "./h.js";

export default a;

export const {
	i = function i2(j) {},
	[a]: k,
	l: [m = (n) => n],
} = {};
export { n } from "./n.js";
export { j } from "./j.js";
export { i2 } from "./i2.js";
