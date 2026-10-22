/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

defineComponent({
  beforeDestroy() {},
});

createApp({
  destroyed() {},
});
