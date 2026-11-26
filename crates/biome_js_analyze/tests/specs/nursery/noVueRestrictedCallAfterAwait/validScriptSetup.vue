<!-- should not generate diagnostics -->
<script setup>
import { useI18n } from "vue-i18n";

await doSomething();
const { t } = useI18n();
</script>
