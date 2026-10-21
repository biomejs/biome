<!-- should not generate diagnostics -->
<script>
import { useI18n } from "vue-i18n";

export default {
	async setup() {
		await doSomething();
		useI18n();
	},
};
</script>
