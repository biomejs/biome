/* should not generate diagnostics */

declare let t: number | undefined;
console.log(t);

declare module "my-module" {
  let value: string;
  export = value;
}
