/* should not generate diagnostics */
import { defineComponent } from "vue";

const options = {
  model: {
    prop: "checked",
    event: "change",
  },
};

export default {
  model: {
    prop: "checked",
  },
};

defineComponent({
  model() {
    return {};
  },
});
