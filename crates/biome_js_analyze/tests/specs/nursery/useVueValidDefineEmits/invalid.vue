<!-- should generate diagnostics -->
<script setup lang="ts">
import { imported } from "./events";

const local = { notify: null };
function validate(payload: string) {
	return payload.length > 0;
}
class Payload {}

defineEmits<(e: "notify") => void>({ submit: null });
defineEmits(local);
defineEmits({ ...local });
defineEmits({ notify: validate, submit: (payload) => payload instanceof Payload });
defineEmits({ local });
defineEmits([imported, local.notify]);
const emit = defineEmits();
const typed = defineEmits(["notify"]) as (event: "notify") => void;
(defineEmits(["notify"]));
</script>
