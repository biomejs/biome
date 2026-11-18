<script>
/* should generate diagnostics */
export default {
  data: function () {
    return {
      copy: this.foo,
    };
  },
  computed: {
    foo() {
      return 1;
    },
  },
};
</script>
