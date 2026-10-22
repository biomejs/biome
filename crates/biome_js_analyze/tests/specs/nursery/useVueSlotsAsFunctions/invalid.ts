/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  render() {
    (this.$slots.foo as any).length;
    this.$slots.bar!.length;
  },
});
