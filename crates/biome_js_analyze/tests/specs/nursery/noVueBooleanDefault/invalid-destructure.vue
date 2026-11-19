<script setup lang="ts">
/* should generate diagnostics */
const {
  foo = true,
  bar: renamedBar = false,
} = defineProps<{ foo?: boolean; bar?: boolean }>();
</script>
