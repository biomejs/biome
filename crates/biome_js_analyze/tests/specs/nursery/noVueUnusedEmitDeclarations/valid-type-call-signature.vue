<!-- should not generate diagnostics -->
<script setup lang="ts">
const emit = defineEmits<{
  (e: "foo"): void;
}>();
const change = () => emit("foo");
</script>
