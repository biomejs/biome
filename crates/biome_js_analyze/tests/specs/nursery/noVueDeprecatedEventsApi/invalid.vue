<!-- should generate diagnostics -->
<script>
export default {
  mounted() {
    this.$on("start", function (args) {
      console.log("start", args);
    });
    this.$off("start");
    this.$once("start", () => {});
    this["$on"]("start", handler);
    this?.$on("start");
    (this?.$off)("start");
    (this.$once)("start");

    const vm = this;
    vm.$on("start", handler);
  },
  methods: {
    stop() {
      this.$off();
    },
  },
};
</script>
