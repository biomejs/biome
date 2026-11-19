<!-- should generate diagnostics -->
<script setup lang="ts">
const emit = defineEmits<{
  foo: [string, number];
  bar: [];
}>();
const change = () => emit("foo");
</script>
