<!-- should not generate diagnostics -->
<script>
import { useI18n, useOther } from "vue-i18n";
import { useI18n as useI18nFromElsewhere } from "other";
import { bar } from "foo";
import { useLocal } from "../local";

export default {
	async setup() {
		useI18n();
		bar.baz();
		await useI18n();

		await doSomething();

		useOther();
		useI18nFromElsewhere();
		bar();
		bar.qux();
		bar.baz.qux();
		useLocal();

		// Nested functions run later, not as part of `setup()`.
		onClick(() => {
			useI18n();
		});
		function later() {
			useI18n();
		}

		{
			const useI18n = () => {};
			useI18n();
		}
	},
	async created() {
		await doSomething();
		useI18n();
	},
};
</script>
