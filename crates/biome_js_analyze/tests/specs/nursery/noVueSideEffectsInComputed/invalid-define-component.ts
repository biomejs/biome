/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  computed: {
    reversed(): string[] {
      return (this.items as string[]).reverse();
    },
    asserted(): string[] {
      return (<string[]>this.items).sort();
    },
  },
});
