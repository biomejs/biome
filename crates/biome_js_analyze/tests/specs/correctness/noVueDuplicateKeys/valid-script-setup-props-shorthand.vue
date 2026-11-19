<!-- should not generate diagnostics -->
<script setup>
const props = defineProps(['foo']);
const foo = useSomething({ props });
</script>
