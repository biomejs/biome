/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

defineComponent({
  render() {
    return this.$slots.default.length;
  },
});

createApp({
  render() {
    return [...this.$slots.default];
  },
});
