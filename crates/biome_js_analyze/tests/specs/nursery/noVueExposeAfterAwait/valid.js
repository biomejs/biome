/* should not generate diagnostics */
import { defineComponent } from "vue";

defineComponent({
	async setup(props, context) {
		context.expose({});
		await doSomething();
	},
});

defineComponent({
	async setup(props, { expose }) {
		expose({});
	},
});

// Not the `setup()` function.
defineComponent({
	async _setup(props, { expose }) {
		await doSomething();
		expose({});
	},
});

// The `await` is inside a nested function, so it doesn't pause `setup()`.
defineComponent({
	async setup(props, { expose }) {
		const load = async () => {
			await doSomething();
		};
		expose({});
	},
});

// The call is inside a nested function, which runs later.
defineComponent({
	async setup(props, { expose }) {
		await doSomething();
		onReady(() => {
			expose({});
		});
	},
});

// `expose` is called before the pause.
defineComponent({
	async setup(props, { expose }) {
		await expose({});
	},
});

// Shadowed by a local variable.
defineComponent({
	async setup(props, { expose }) {
		await doSomething();
		{
			const expose = () => {};
			expose({});
		}
	},
});

// Not a function from the setup context.
defineComponent({
	async setup(props, { emit }) {
		await doSomething();
		expose({});
		emit("change");
	},
});

defineComponent({
	async setup(props, context) {
		await doSomething();
		context.emit("change");
		other.expose({});
	},
});

// Not a Vue component.
export default {
	async setup(props, { expose }) {
		await doSomething();
		expose({});
	},
};
