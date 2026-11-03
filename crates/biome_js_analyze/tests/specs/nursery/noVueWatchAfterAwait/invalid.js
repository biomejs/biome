/* should generate diagnostics */
import { createApp, defineComponent, watch, watchEffect } from "vue";

defineComponent({
	setup: async function () {
		await doSomething();
		watch(source, () => {});
	},
});

defineComponent({
	setup: async () => {
		await doSomething();
		watchEffect(() => {});
	},
});

createApp({
	async setup() {
		await doSomething();
		watch(source, () => {});
	},
});

// Each watcher after the first `await` is reported.
defineComponent({
	async setup() {
		await doSomething();
		watch(source, () => {});
		await doSomething();
		watch(source, () => {});
	},
});

// The `await` is inside a block that comes before the watcher.
defineComponent({
	async setup() {
		if (shouldLoad) {
			await doSomething();
		}
		watchEffect(() => {});
	},
});

// The `await` is in the arguments of an earlier call.
defineComponent({
	async setup() {
		const data = useData(await doSomething());
		watchEffect(() => {});
	},
});

// The watcher is created after its argument is awaited.
defineComponent({
	async setup() {
		watch(await loadSource(), () => {});
	},
});

defineComponent({
	async setup() {
		for await (const item of items) {
			watch(item, () => {});
		}
		watchEffect(() => {});
	},
});
