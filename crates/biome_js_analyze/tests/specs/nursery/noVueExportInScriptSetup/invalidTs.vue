<!-- should generate diagnostics -->
<script setup lang="ts">
export const Foo = {};
export enum Bar {}
export {};
export abstract class Baz {}
export namespace Qux {
  export const value = 1;
}
const value = 1;
export { type Foo as FooType, value };
export { type X, Y } from "foo";
</script>
