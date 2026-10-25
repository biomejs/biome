/* should not generate diagnostics */
const emitter = { $emit() {}, emit() {} };
emitter.$emit("change", value1, value2);

function setup(props, { emit }) {
  emit("change", value1, value2);
}
