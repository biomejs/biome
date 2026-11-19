<script>
/* should not generate diagnostics */
export default {
  emits: {
    foo() {
      return true;
    },
    bar: function (e) {
      return true;
    },
    baz: (e) => {
      return e;
    },
    baz2: (e) => e,
    qux() {
      if (foo) {
        return true;
      } else {
        return false;
      }
    },
    quux: null,
    corge(evt) {
      return evt;
    },
    collectsInNestedArrow() {
      const options = [];
      this.matches.forEach((match) => {
        options.push(match);
      });
      return options;
    },
    collectsInNestedFunction() {
      const options = [];
      this.matches.forEach(function (match) {
        options.push(match);
      });
      return options;
    },
    returnsInLoop(items) {
      for (const item of items) {
        if (item) {
          return true;
        }
      }
      return false;
    },
    a() {
      return 1n;
    },
    b: function (e) {
      return 1;
    },
    c: (e) => {
      return "a";
    },
    d: () => `a`,
    nonZeroNumber: () => 0.5,
    e: () => void 0 || true,
    namedValidator: validate,
  },
};
</script>
