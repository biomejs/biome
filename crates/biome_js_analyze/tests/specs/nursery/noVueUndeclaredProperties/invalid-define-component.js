/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  props: { title: String },
  computed: {
    upper() {
      return this.titel.toUpperCase();
    },
  },
  methods: {
    reset() {
      this.missing = 0;
      this.undeclaredMethod();
    },
  },
});
