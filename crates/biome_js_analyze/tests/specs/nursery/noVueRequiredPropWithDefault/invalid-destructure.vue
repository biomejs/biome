<!-- should generate diagnostics -->
<script setup lang="ts">
const { name = "World", label: renamed = "Label" } = defineProps<{
  name: string;
  label: string;
}>();
</script>
