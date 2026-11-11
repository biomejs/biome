<script setup lang="ts">
/* should not generate diagnostics */
withDefaults(defineProps<{ flag?: boolean | undefined; other?: true | false }>(), {});
const { enabled } = defineProps<{ enabled?: boolean | undefined }>();
</script>
