<!-- should not generate diagnostics -->
<script setup lang="ts">
export { type Foo } from "foo";
export type { Bar } from "bar";
export type * from "baz";
export type * as types from "baz";
export type Qux = {};
export interface Quux {}
type Local = string;
const value = 1;
export { type Local };
export type { Local as LocalAlias };
export declare const declared: number;
export = value;
export as namespace Lib;
declare module "foo" {
  export const inner: number;
}
</script>
