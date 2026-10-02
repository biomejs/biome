/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    name: {
      required: true,
      default: "Hello",
    },
  },
});
