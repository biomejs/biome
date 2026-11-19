<!-- should generate diagnostics -->
<script>
import { useI18n, useI18n as useTranslation } from "vue-i18n";
import * as i18n from "vue-i18n";
import { bar } from "foo";
import useDefault, { default as renamedDefault } from "default-export";
import explicitDefault from "explicit-default";
import { useLocal } from "./local";
import { useThing } from "with-message";

export default {
	async setup() {
		await doSomething();

		useI18n();
		useTranslation();
		i18n.useI18n();
		i18n["useI18n"]();
		bar.baz();
		useDefault();
		renamedDefault();
		explicitDefault();
		useLocal();
		useThing();

		useI18n?.();
		(useI18n)();
		useI18n(await doSomething());

		if (ready) {
			useI18n();
		}
	},
};
</script>
