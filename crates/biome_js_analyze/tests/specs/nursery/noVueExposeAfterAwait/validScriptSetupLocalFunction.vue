<!-- should not generate diagnostics -->
<script setup>
await doSomething();
{
	defineExpose({});
}
function defineExpose() {}
</script>
