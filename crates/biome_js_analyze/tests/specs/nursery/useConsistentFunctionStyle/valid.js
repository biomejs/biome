// should not generate diagnostics
const foo = function() {};
let bar = () => {};
var generator = async function* () {};
export const exported = () => {};
export default function named() {}
run(function() {}, () => {});
const object = { method() {}, property: function() {} };
object.method = function() {};

