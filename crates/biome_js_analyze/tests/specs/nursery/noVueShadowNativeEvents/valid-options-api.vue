<script>
/* should not generate diagnostics */
function outside() {
  this.$emit("click");
}

export default {
  emits: ["buttonClick", "welcome", unknown, ...others],
  methods: {
    onClick() {
      this.$emit("welcome");
      this.$emit(eventName);
      this.$emit();
      this.$emit(1);
      this.$emit(`cl${"ick"}`);
      this.$fire("click");
      this.bar.$emit("click");
      $emit("click");
      this.$emit;
    },
  },
};
</script>
