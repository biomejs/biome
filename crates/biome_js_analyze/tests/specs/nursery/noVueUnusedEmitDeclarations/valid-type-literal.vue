<!-- should not generate diagnostics -->
<script setup lang="ts">
const emit = defineEmits<{
  foo: [string, number];
}>();
const change = () => emit("foo");
</script>
