/* should not generate diagnostics */
import { defineComponent } from "vue";

export const SetupDestructure = defineComponent({
  props: {
    count: Number,
  },
  setup({ count }) {
    count = count + 1;
    return { count };
  },
});

const notAComponent = {
  props: ["value"],
  methods: {
    update() {
      this.value = "test";
    },
  },
};
