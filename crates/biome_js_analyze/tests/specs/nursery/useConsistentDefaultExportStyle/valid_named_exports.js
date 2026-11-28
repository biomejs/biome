/* should not generate diagnostics */
export function foo() {}
export class Foo {}
const bar = () => {};
export { bar };
const baz = 1;
export { baz as default };
export { qux as default } from "qux";
