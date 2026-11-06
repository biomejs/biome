<!-- should not generate diagnostics -->
<script>
import { onBeforeUnmount, onMounted, onUnmounted } from "vue";

export default {
	async setup() {
		onMounted(() => {});
		onBeforeUnmount(() => {});

		await doSomething();

		// The component is passed explicitly as the second argument.
		onUnmounted(() => {}, instance);

		// Code in a nested function runs when the function is called, not during `setup()`.
		const register = () => {
			onMounted(() => {});
		};
		function registerLater() {
			onMounted(() => {});
		}
		const handlers = {
			register() {
				onMounted(() => {});
			},
		};
	},
};
</script>
