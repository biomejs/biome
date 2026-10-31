<!-- should generate diagnostics -->
<script>
import {
	onActivated,
	onBeforeMount,
	onBeforeUnmount,
	onBeforeUpdate,
	onDeactivated,
	onErrorCaptured,
	onMounted,
	onRenderTracked,
	onRenderTriggered,
	onUnmounted,
	onUpdated,
	onMounted as mounted,
} from "vue";
import * as Vue from "vue";

export default {
	async setup() {
		await doSomething();

		onActivated(() => {});
		onBeforeMount(() => {});
		onBeforeUnmount(() => {});
		onBeforeUpdate(() => {});
		onDeactivated(() => {});
		onErrorCaptured(() => {});
		onMounted(() => {});
		onRenderTracked(() => {});
		onRenderTriggered(() => {});
		onUnmounted(() => {});
		onUpdated(() => {});

		onMounted?.(() => {});
		mounted(() => {});
		Vue.onMounted(() => {});
		(onMounted)(() => {});

		if (ready) {
			onMounted(() => {});
		}
	},
};
</script>
