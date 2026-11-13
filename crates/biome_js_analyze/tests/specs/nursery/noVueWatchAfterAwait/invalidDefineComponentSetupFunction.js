/* should generate diagnostics */
import { defineComponent, watch, watchEffect } from "vue";

defineComponent(
	async (props, context) => {
		await doSomething();
		watch(source, () => {});
	},
	{ props: ["source"] },
);

defineComponent(
	async function (props) {
		await doSomething();
		watchEffect(() => {});
	},
	{ props: ["source"] },
);
