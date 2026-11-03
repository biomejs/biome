<!-- should generate diagnostics -->
<script>
import { watch, watchEffect, watch as observe } from "vue";
import * as Vue from "vue";

export default {
	async setup() {
		await doSomething();

		watch(source, () => {});
		watchEffect(() => {});
		watch?.(source, () => {});
		observe(source, () => {});
		Vue.watch(source, () => {});
		Vue.watchEffect(() => {});
		(watch)(source, () => {});

		if (ready) {
			watch(source, () => {});
		}

		// The returned function is discarded.
		(watch(source, () => {}));
		await watch(source, () => {});
		return watchEffect(() => {});
	},
};
</script>
