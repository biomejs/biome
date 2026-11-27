/* should generate diagnostics */
export default function foo() {}
const bar = 1;
export { bar as default };
export { default } from "./baz.js";
export { qux as "default" } from "./qux.js";
