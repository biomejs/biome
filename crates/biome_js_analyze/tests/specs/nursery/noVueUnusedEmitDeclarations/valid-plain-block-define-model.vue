<!-- should not generate diagnostics -->
<script>
export default {
  emits: ["update:foo"],
};
</script>

<script setup>
const foo = defineModel("foo");
</script>
