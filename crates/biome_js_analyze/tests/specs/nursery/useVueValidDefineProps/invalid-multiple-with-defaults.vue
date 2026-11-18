<!-- should generate diagnostics -->
<script setup lang="ts">
const props = withDefaults(defineProps<{ msg?: string }>(), { msg: "hello" });
defineProps();
</script>
