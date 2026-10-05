/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

defineComponent({
  render() {
    return this.$scopedSlots;
  },
});

createApp({
  render() {
    this.$scopedSlots = {};
  },
});
