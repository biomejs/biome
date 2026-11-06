<script>
/* should generate diagnostics */
import { h } from "vue";

export default {
	render() {
		if (this.visible) {
			return h("div", "hello");
		}
	},
};
</script>
