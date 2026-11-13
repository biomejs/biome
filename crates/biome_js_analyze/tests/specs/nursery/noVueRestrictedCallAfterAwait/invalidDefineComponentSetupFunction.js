/* should generate diagnostics */
import { defineComponent } from "vue";
import { useI18n } from "vue-i18n";

defineComponent(
	async (props, context) => {
		await doSomething();
		useI18n();
	},
	{ props: ["source"] },
);

defineComponent(
	async function (props) {
		await doSomething();
		useI18n();
	},
	{ props: ["source"] },
);
