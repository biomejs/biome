<!-- should generate diagnostics -->
<script>
export default {
	async setup(props, { expose }) {
		await doSomething();
		expose({});
	},
};
</script>
