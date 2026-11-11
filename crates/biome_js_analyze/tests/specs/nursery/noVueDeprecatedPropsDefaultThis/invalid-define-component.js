/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    a: String,
    b: {
      type: Array,
      default() {
        return [this.a];
      },
    },
  },
});
