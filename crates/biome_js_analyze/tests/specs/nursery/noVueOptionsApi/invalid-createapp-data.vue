<script>
// should generate diagnostics
// createApp with Options API is not supported in Vapor Mode
import { createApp } from "vue";

createApp({
	data() {
		return { count: 0 };
	},
}).mount("#app");
</script>
