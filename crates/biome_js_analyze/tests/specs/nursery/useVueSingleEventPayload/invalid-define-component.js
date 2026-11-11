/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  emits: {
    change: (value1, value2) => true,
  },
  setup(props, { emit }) {
    emit("change", value1, value2);
  },
});
