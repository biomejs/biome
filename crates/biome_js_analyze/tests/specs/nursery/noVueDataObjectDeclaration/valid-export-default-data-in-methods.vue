<script>
/* should not generate diagnostics */
// export default with data property on an object defined in methods
export default {
	methods: {
		foo() {
			const bar = {
				data: {}
			}
		}
	}
}
</script>
