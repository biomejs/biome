/* should not generate diagnostics */
export default {
  data() {
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
