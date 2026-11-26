/* should not generate diagnostics */
import { defineComponent, onMounted } from "vue";

// Vue runs `sharedSetup` and ignores the `setup` option.
defineComponent(sharedSetup, {
	async setup() {
		await doSomething();
		onMounted(() => {});
	},
});
