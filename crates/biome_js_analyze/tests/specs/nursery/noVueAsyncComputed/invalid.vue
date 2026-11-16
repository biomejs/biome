<!-- should generate diagnostics -->
<script>
import { nextTick } from "vue";

export default {
	computed: {
		asyncMethod: async function () {
			return await someFunc();
		},
		asyncShorthand: async () => {
			return someFunc();
		},
		asyncExpressionArrow: async () => 1,
		async asyncMember() {
			return 1;
		},
		newPromise() {
			return new Promise((resolve) => resolve(1));
		},
		newWindowPromise() {
			return new window.Promise((resolve) => resolve(1));
		},
		then() {
			return bar.then((response) => response);
		},
		optionalThen() {
			return bar?.then?.((response) => response);
		},
		parenthesizedThen() {
			return (bar?.then)?.((response) => response);
		},
		catchMethod() {
			return bar.catch((error) => error);
		},
		finallyMethod() {
			return bar.finally(() => {});
		},
		chainedCatch() {
			return myFunc().catch("default");
		},
		computedMemberThen() {
			return bar["then"](() => {});
		},
		promiseAll() {
			return Promise.all([]);
		},
		promiseAllSettled() {
			return Promise.allSettled([]);
		},
		promiseAny() {
			return Promise.any([]);
		},
		promiseRace() {
			return Promise.race([]);
		},
		promiseReject() {
			return Promise.reject(new Error());
		},
		promiseResolve() {
			return Promise.resolve([]);
		},
		promiseTry() {
			return Promise.try(() => 1);
		},
		promiseWithResolvers() {
			return Promise.withResolvers();
		},
		globalThisPromise() {
			return globalThis.Promise.resolve(1);
		},
		getter: {
			get() {
				return Promise.resolve([]);
			},
			set(value) {
				this.value = value;
			},
		},
		arrowGetter: {
			get: () => Promise.resolve([]),
		},
		timers() {
			setTimeout(() => {}, 0);
			window.setTimeout(() => {}, 0);
			setInterval(() => {}, 0);
			window.setInterval(() => {}, 0);
			setImmediate(() => {});
			window.setImmediate(() => {});
			requestAnimationFrame(() => {});
			window.requestAnimationFrame(() => {});
			globalThis.setTimeout(() => {});
			return 1;
		},
		optionalTimers() {
			setTimeout?.(() => {}, 0);
			window?.setTimeout?.(() => {}, 0);
			(window?.setInterval)?.(() => {}, 0);
			return 1;
		},
		nextTicks() {
			this.$nextTick(() => {});
			nextTick(() => {});
			return 1;
		},
		async awaitedNextTicks() {
			await this.$nextTick();
			await nextTick();
			return 1;
		},
		nestedInControlFlow() {
			if (this.enabled) {
				for (const item of this.items) {
					item.load().then(() => {});
				}
			}
			return 1;
		},
	},
};
</script>
