/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

defineComponent({
  mounted() {
    this.$on("start", () => {});
    this.$off!("start");
  },
});

createApp({
  mounted() {
    this.$once("start", () => {});
  },
});
