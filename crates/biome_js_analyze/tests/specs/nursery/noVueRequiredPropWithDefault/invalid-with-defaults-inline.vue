<!-- should generate diagnostics -->
<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    name: string;
    age?: number;
  }>(),
  {
    name: "World",
  },
);
</script>
