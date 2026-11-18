<!-- should generate diagnostics -->
<script>
import { nextTick as nt } from "vue";
import * as Vue from "vue";

export default {
	async mounted() {
		// The Promise is ignored.
		nt();
		Vue.nextTick();
		this.$nextTick();

		// Too many arguments.
		nt(callback, anotherCallback);
		Vue.nextTick(callback, anotherCallback);
		this.$nextTick(callback, anotherCallback);

		// Not called.
		nt.then(callback);
		Vue.nextTick.then(callback);
		this.$nextTick.then(callback);
		await nt;
		await Vue.nextTick;
		await this.$nextTick;
		Promise.all([nt, someOtherPromise]);

		// Both a callback and the Promise.
		nt(callback).then(anotherCallback);
		Vue.nextTick(callback).then(anotherCallback);
		this.$nextTick(callback).then(anotherCallback);
		await nt(callback);
		await Vue.nextTick(callback);
		await this.$nextTick(callback);
	},
	updated() {
		// No `await` fix outside of an async function.
		nt();
		this.$nextTick();
		return this.$nextTick;
	},
};
</script>
