<!-- should generate diagnostics -->
<script>
export default {
	async setup(props, context) {
		await doSomething();
		context.expose({});
		context?.expose({});
		context.expose?.({});
		context["expose"]({});
		(context).expose({});
	},
};
</script>
