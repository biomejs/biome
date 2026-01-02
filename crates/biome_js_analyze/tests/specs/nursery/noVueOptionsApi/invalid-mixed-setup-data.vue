<script>
// Mixed setup() with Options API is not supported in Vapor Mode
export default {
	setup() {
		const someRef = ref(0);
		return { someRef };
	},
	data() {
		return { count: 0 };
	},
};
</script>
