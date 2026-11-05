/* should not generate diagnostics */
import { defineComponent } from "vue";

const options = {
  init() {},
};

defineComponent(options);

configure({
  init() {},
});
