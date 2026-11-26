<script setup lang="ts">
/* should not generate diagnostics */
interface Props {
  foo?: number;
}

const props = defineProps<Props>();
</script>
