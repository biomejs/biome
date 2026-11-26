<script setup lang="ts">
/* should not generate diagnostics */
const { cb = () => {} } = withDefaults(
  defineProps<{ cb?: { (): void }; factory?: { (): number }; create?: { new (): object } }>(),
  { factory: () => 1 },
);
</script>
