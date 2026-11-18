/* should not generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    user: Object,
    items: { type: Array },
  },
});
