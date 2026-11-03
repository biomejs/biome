<!-- should not generate diagnostics -->
<script setup lang="ts">
const props = withDefaults(defineProps<{ msg?: string }>(), { msg: "hello" });
</script>
