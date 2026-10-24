/* should not generate diagnostics */
import { defineComponent } from "vue";

const notAComponent = {
  beforeRouteEnter() {
    this.method();
  },
};

export default defineComponent({
  beforeRouteEnter(to, from, next) {
    next((vm) => vm.method());
  },
});
