/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  computed: {
    total() {
      return 1 + 2;
    },
  },
  methods: {
    log() {
      console.log(this.total());
    },
  },
});
