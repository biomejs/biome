<!-- should not generate diagnostics -->
<script setup lang="ts">
import { validate } from "./validators";

type Payload = string;
const limit = 10;

const emit = defineEmits({
	notify: null,
	submit: validate,
	change(payload: Payload) {
		const isShort = payload.length < 5;
		return isShort;
	},
	resize: (size: typeof limit) => size > 0,
	close: (reason) => typeof reason === "string",
});

function notify() {
	emit("notify");
}
</script>
