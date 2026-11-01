/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    foo: String,
    bar: {
      type: Number,
    },
  },
});
