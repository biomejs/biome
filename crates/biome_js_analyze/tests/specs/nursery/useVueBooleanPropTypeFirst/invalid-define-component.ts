/* should generate diagnostics */
import { defineComponent, type PropType } from "vue";

export default defineComponent({
  props: {
    a: [String, Boolean],
    b: { type: [String, Boolean] as PropType<string | boolean> },
  },
});
