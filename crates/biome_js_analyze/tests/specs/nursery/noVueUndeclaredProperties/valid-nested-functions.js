/* should not generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  data() {
    return { count: 0 };
  },
  methods: {
    later() {
      setTimeout(() => {
        this.count++;
      });
      return function () {
        return this.unrelated;
      };
    },
  },
});
