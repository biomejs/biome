/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  data() {
    return {
      copy: this.foo,
    };
  },
  computed: {
    foo() {
      return 1;
    },
  },
});
