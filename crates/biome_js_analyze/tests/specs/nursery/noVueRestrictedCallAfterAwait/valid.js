/* should not generate diagnostics */
import { defineComponent } from "vue";
import { useI18n } from "vue-i18n";

defineComponent({
	async setup() {
		const load = async () => {
			await doSomething();
		};
		useI18n();
		await load();
	},
});

// Not a component.
async function setup() {
	await doSomething();
	useI18n();
}

const options = {
	async setup() {
		await doSomething();
		useI18n();
	},
};
