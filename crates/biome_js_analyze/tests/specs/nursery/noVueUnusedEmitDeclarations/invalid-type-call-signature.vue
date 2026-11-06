<!-- should generate diagnostics -->
<script setup lang="ts">
const emit = defineEmits<{
  (e: "foo" | "bar"): void;
  (e: "baz", id: number): void;
}>();
const change = () => emit("foo");
</script>
