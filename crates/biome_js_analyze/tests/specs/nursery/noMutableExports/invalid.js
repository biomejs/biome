/* should generate diagnostics */
export let a = 1;
export var b = 1;
export let c, d;
export let { e, f } = {};

let g = 1;
export { g };

var h = 1;
export { h as hAlias, h as "hString" };

let [i] = [];
export { i };

for (var j of []) {}
export { j };
