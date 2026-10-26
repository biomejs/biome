<script>
/* should not generate diagnostics */
const model = { prop: "checked", event: "change" };

export default {
  model,
  props: ["modelValue", "model"],
  emits: ["update:modelValue"],
  data() {
    return {
      model: {
        prop: "checked",
      },
    };
  },
};
</script>
