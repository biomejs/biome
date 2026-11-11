<script setup lang="ts">
/* should generate diagnostics */
const { foo, bar, baz = "baz" } = defineProps<{
  foo?: number;
  bar: number;
  baz?: string;
}>();
</script>
