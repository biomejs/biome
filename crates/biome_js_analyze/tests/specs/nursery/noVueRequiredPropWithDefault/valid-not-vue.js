/* should not generate diagnostics */
export default {
  props: {
    name: {
      required: true,
      default: "Hello",
    },
  },
};

function withDefaults(props, defaults) {}
function defineProps() {}
withDefaults(defineProps(), { name: "World" });
