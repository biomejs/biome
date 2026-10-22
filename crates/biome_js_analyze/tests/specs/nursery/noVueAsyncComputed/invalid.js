/* should generate diagnostics */
import { computed, createApp, defineComponent, computed as vueComputed } from "vue";
import * as Vue from "vue";

const asyncArrow = computed(async () => {
	return await someFunc();
});
const asyncExpression = computed(async () => await someFunc());
const asyncFunction = computed(async function () {
	return await someFunc();
});
const asyncWithoutAwait = computed(async () => {
	bar();
});
const newPromise = computed(() => {
	return new Promise((resolve) => resolve(1));
});
const then = computed(() => bar.then((response) => response));
const promiseAll = computed(() => {
	return Promise.all([]);
});
const getterArrow = computed({
	get: () => {
		return Promise.resolve([]);
	},
});
const getterMethod = computed({
	get() {
		return Promise.resolve([]);
	},
	set(value) {},
});
const timers = computed(() => {
	setTimeout(() => {}, 0);
	window.requestAnimationFrame(() => {});
	return 1;
});
const renamed = vueComputed(() => Promise.resolve(1));
const namespaced = Vue.computed(() => Promise.resolve(1));
const namespacedNextTick = computed(() => {
	Vue.nextTick(() => {});
	return 1;
});
const nestedComputed = computed(() => {
	const inner = computed(() => Promise.resolve(1));
	return inner.value;
});

defineComponent({
	computed: {
		foo() {
			return Promise.resolve(1);
		},
	},
});

defineComponent({
	setup() {
		const foo = computed(async () => 1);
		return { foo };
	},
});

createApp({
	computed: {
		async foo() {
			return 1;
		},
	},
});
