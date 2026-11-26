<!-- should generate diagnostics -->
<script>
export default {
	async setup(props, { expose: exposeProperties }) {
		await doSomething();
		exposeProperties({});
		(exposeProperties)({});
	},
};
</script>
