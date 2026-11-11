/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

defineComponent({
  render() {
    return this.$listeners;
  },
});

createApp({
  render() {
    this.$listeners = {};
  },
});
