/* should not generate diagnostics */
import { defineComponent } from "other-library";

export default defineComponent({
  beforeRouteEnter() {
    this.method();
  },
});
