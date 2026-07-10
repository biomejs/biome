<script>
// should generate diagnostics
// Options API: computed is not supported in Vapor Mode
export default {
	computed: {
		doubleCount() {
			return this.count * 2;
		},
	},
};
</script>
