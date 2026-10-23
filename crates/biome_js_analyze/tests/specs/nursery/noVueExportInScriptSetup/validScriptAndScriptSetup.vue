<!-- should not generate diagnostics -->
<script>
export * from "foo";
export default {};
export class A {}
</script>
<script setup>
let foo;
defineExpose({ foo });
</script>
