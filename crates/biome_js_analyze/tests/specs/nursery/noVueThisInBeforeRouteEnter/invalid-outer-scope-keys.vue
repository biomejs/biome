<script>
/* should generate diagnostics */
export default {
  beforeRouteEnter() {
    const object = {
      [this.methodKey]() {},
      get [this.getterKey]() {
        return 1;
      },
      [this.propertyKey]: function () {},
    };
    class Local extends this.Base {
      [this.classKey]() {}
      [this.fieldKey] = 1;
    }
  },
};
</script>
