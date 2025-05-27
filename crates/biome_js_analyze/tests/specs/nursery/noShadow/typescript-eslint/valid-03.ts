/* should not generate diagnostics */
export type ArrayInput<Func> = Func extends (arg0: Array<infer T>) => any
  ? T[]
  : Func extends (...args: infer T) => any
    ? T
    : never;
    `,
    `
function foo() {
  var Object = 0;
}
