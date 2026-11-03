<!-- should generate diagnostics -->
<script setup>
import { nextTick } from "vue";

nextTick();
(nextTick)();
nextTick().catch(handleError);
void nextTick();
nextTick;
condition ? nextTick : other;
nextTick["then"](callback);
await nextTick(callback);
const promise = nextTick(callback);
other(), nextTick();
</script>
