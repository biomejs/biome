/* should not generate diagnostics */
import { ref } from "vue";

const counter = ref(0);
const names = ref(new Set());

function incrementCounter() {
  counter.value++;
  return counter.value;
}
