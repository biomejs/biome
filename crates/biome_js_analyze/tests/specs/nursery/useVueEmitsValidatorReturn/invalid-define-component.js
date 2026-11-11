/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  emits: {
    change(value) {
      console.log(value);
    },
  },
});
