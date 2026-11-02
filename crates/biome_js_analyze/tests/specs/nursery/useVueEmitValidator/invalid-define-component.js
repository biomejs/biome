/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  emits: ["foo"],
});

export const Other = defineComponent({
  emits: {
    foo: null,
    bar: (payload) => {},
  },
});
