<!-- should not generate diagnostics -->
<script setup>
async function load() {
	await doSomething();
}
defineExpose({});
</script>
