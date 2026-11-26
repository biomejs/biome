/* should generate diagnostics */
export const a = 1;
function b() {}
export { b as a };

const c = 1;
export { c };
export { c };

const d = 1;
export { d, d };

const e = 1;
export { e as f, e as f };

export { g } from "./g.js";
export { h as g } from "./h.js";

export * as i from "./i.js";
export const i = 1;

const j = 1;
export { j as "k" };
export { j as k };

const l = 1;
export { l as "m" };
export { l as m };

export const { n, o: [p] } = {};
export { n as p };

export function q() {}
export * as q from "./q.js";

export class r {}
export { s as r } from "./s.js";
