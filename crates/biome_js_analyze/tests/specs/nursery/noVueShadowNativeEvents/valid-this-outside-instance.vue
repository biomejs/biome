<!-- should not generate diagnostics -->
<!-- `this` in an arrow function method or in a nested function is not the component. -->
<script>
export default {
  methods: {
    onArrow: () => this.$emit("click"),
    onNested() {
      list.forEach(function () {
        this.$emit("click");
      });
    },
  },
};
</script>
