/* should not generate diagnostics */
import { defineComponent } from "vue";
import { computed } from "./my-computed";

export default defineComponent({
  setup() {
    const state = { count: 0 };
    const test = computed(() => state.count++);
    return { test };
  },
});
