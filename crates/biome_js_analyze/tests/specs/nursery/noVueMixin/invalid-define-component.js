/* should generate diagnostics */
import { defineComponent } from "vue";
import counterMixin from "./counterMixin";

export default defineComponent({
  mixins: [counterMixin],
});
