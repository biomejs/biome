<!-- should not generate diagnostics -->
<script>
export default {
	async setup(props, { expose }) {
		expose({});
		await doSomething();
	},
};
</script>
