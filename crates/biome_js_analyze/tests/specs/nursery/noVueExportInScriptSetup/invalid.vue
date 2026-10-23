<!-- should generate diagnostics -->
<script setup>
export * from "foo";
export * as ns from "foo";
export default {};
export class A {}
export const test = "123";
export function foo() {}
const a = 1;
export { a };
export { a as b };
export { fao } from "bar";
export {};
</script>
