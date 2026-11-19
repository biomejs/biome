/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    foo: {
      type: Boolean,
      default: true,
    },
  },
});
