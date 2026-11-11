/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

defineComponent({
  model: {
    prop: "value",
    event: "input",
  },
});

defineComponent((props) => {}, {
  model: {},
});

createApp({
  model: ({
    event: "change",
  }),
});
