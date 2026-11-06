/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

export const Options = defineComponent({
  emits: ["click"],
  setup(props, { emit }) {
    emit("keydown");
  },
  methods: {
    onClick() {
      this.$emit("change");
    },
  },
});

export const Functional = defineComponent(
  (props, { emit }) => {
    emit("input");
  },
  { emits: ["input"] },
);

createApp({
  emits: ["submit"],
});
