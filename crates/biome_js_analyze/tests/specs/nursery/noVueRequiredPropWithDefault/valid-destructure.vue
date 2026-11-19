<!-- should not generate diagnostics -->
<script setup lang="ts">
const { name = "World", age } = defineProps<{
  name?: string;
  age: number;
}>();
</script>
