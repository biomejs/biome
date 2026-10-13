/* should generate diagnostics */
import { createApp, defineComponent, onMounted, onUnmounted } from "vue";

defineComponent({
	setup: async function () {
		await doSomething();
		onMounted(() => {});
	},
});

defineComponent({
	setup: async () => {
		await doSomething();
		onMounted(() => {});
	},
});

createApp({
	async setup() {
		await doSomething();
		onMounted(() => {});
	},
});

// The `await` is inside a block that comes before the hook.
defineComponent({
	async setup() {
		if (shouldLoad) {
			await doSomething();
		}
		onMounted(() => {});
	},
});

// The `await` is in the arguments of an earlier call.
defineComponent({
	async setup() {
		const data = useData(await doSomething());
		onMounted(() => {});
	},
});

// The hook is called after its argument is awaited.
defineComponent({
	async setup() {
		onMounted(await loadHandler());
	},
});

defineComponent({
	async setup() {
		for await (const item of items) {
			onMounted(() => {});
		}
		onUnmounted(() => {});
	},
});
