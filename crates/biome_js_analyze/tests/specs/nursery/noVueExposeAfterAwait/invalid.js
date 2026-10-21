/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

defineComponent({
	setup: async function (props, { expose }) {
		await doSomething();
		expose({});
	},
});

defineComponent({
	setup: async (props, context) => {
		await doSomething();
		context.expose({});
	},
});

createApp({
	async setup(props, { expose }) {
		await doSomething();
		expose({});
	},
});

defineComponent(
	async (props, { expose }) => {
		await doSomething();
		expose({});
	},
	{ props: ["value"] },
);

// The `await` is inside a block that comes before the call.
defineComponent({
	async setup(props, { expose }) {
		if (shouldLoad) {
			await doSomething();
		}
		expose({});
	},
});

// `for await` pauses `setup()` too.
defineComponent({
	async setup(props, { expose }) {
		for await (const item of items) {
		}
		expose({});
	},
});

// The argument is computed after the pause.
defineComponent({
	async setup(props, { expose }) {
		expose(await load());
	},
});

// Every call after the pause is reported.
defineComponent({
	async setup(props, { expose }) {
		await doSomething();
		expose({});
		expose({});
	},
});
