<script>
// should generate diagnostics
// defineComponent with Options API is not supported in Vapor Mode
import { defineComponent } from "vue";

export default defineComponent({
	data() {
		return { count: 0 };
	},
});
</script>
