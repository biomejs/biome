/* should generate diagnostics */
import { defineComponent, onMounted, onUnmounted } from "vue";

defineComponent(
	async (props, context) => {
		await doSomething();
		onMounted(() => {});
	},
	{ props: ["source"] },
);

defineComponent(
	async function (props) {
		await doSomething();
		onUnmounted(() => {});
	},
	{ props: ["source"] },
);
