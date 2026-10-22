<!-- should generate diagnostics -->
<script setup lang="ts">
import { computed, nextTick } from "vue";

const asyncArrow = computed(async () => {
	return await someFunc();
});
const asyncFunction = computed(async function () {
	return await someFunc();
});
const newPromise = computed(() => new Promise((resolve) => resolve(1)));
const then = computed(() => bar.then((response) => response));
const getter = computed({
	get() {
		return Promise.resolve([]);
	},
});
const timer = computed(() => {
	setInterval(() => {}, 0);
	return 1;
});
const tick = computed(() => {
	nextTick(() => {});
	return 1;
});
const zod = computed(() => z.number().catch(42));
</script>
