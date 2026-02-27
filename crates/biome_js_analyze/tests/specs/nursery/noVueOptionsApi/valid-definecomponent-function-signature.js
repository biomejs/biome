// should not generate diagnostics
// defineComponent with function signature is Composition API (Vue 3.3+)
import { defineComponent, ref, h } from "vue";

export default defineComponent((props) => {
  const count = ref(0);
  return () => h("div", count.value);
});
