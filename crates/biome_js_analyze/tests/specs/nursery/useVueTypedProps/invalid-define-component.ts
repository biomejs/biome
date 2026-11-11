/* should generate diagnostics */
import { defineComponent, type PropOptions } from "vue";

export default defineComponent({
  props: {
    foo: {} as PropOptions<string>,
  },
});
