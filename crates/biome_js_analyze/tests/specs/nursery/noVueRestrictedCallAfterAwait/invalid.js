/* should generate diagnostics */
import { createApp, defineComponent } from "vue";
import { useI18n } from "vue-i18n";

defineComponent({
	setup: async function () {
		await doSomething();
		useI18n();
	},
});

defineComponent({
	setup: async () => {
		await doSomething();
		useI18n();
	},
});

createApp({
	async setup() {
		await doSomething();
		useI18n();
	},
});

// The `await` is inside a block that comes before the call.
defineComponent({
	async setup() {
		if (shouldLoad) {
			await doSomething();
		}
		useI18n();
	},
});

defineComponent({
	async setup() {
		for await (const item of items) {
		}
		useI18n();
	},
});
