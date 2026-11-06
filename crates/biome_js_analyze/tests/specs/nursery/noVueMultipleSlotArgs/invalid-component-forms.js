/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

defineComponent({
  render() {
    return this.$slots.default(foo, bar);
  },
});

createApp({
  render() {
    return this.$slots.default(...foo);
  },
});
