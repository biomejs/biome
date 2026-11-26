/* should not generate diagnostics */
const emitter = {
	emits: [],
	methods: {
		close() {
			this.$emit("close");
		},
	},
};
