<script setup lang="ts">
/* should generate diagnostics */
const { foo = 1, bar = "bar" } = defineProps<{ foo?: string; bar?: boolean }>();
</script>
