/* should not generate diagnostics */
import { defineComponent, onMounted } from "vue";
import { onMounted as onMountedElsewhere } from "some-library";

defineComponent({
	async setup() {
		onMounted(() => {});
		await doSomething();
	},
});

defineComponent({
	async setup() {
		onMounted(() => {});
	},
});

// The hook is called before `setup()` pauses to wait for its result.
defineComponent({
	async setup() {
		await onMounted(() => {});
	},
});

// An `await` inside a nested function doesn't pause `setup()`.
defineComponent({
	setup() {
		const load = async () => {
			await doSomething();
		};
		onMounted(() => {});
	},
});

defineComponent({
	async setup() {
		const load = async () => {
			await doSomething();
			onMounted(() => {});
		};
	},
});

// Not a function named `setup`.
defineComponent({
	async _setup() {
		await doSomething();
		onMounted(() => {});
	},
});

// Not imported from Vue.
defineComponent({
	async setup() {
		await doSomething();
		onMountedElsewhere(() => {});
	},
});

defineComponent({
	async setup() {
		const onMounted = () => {};
		await doSomething();
		onMounted(() => {});
	},
});

// Not a Vue component.
const options = {
	async setup() {
		await doSomething();
		onMounted(() => {});
	},
};
