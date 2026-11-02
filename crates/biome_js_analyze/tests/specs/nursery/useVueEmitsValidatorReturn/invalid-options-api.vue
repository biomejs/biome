<script>
/* should generate diagnostics */
export default {
  emits: {
    methodWithoutReturn() {
    },
    functionWithoutReturn: function () {
    },
    arrowWithoutReturn: () => {
    },
    arrowReturningFalse: () => false,
    arrowReturningParenthesizedNull: () => (null),
    bareReturn() {
      return;
    },
    nestedFunctionReturn: function () {
      function bar() {
        return this.baz * 2;
      }
      const qux = () => true;
      bar();
    },
    nestedClassReturn() {
      class Foo {
        method() {
          return true;
        }
      }
    },
    onlyFalsyReturns: function () {
      if (a) {
        return false;
      } else if (b) {
        return 0;
      } else if (c) {
        return null;
      } else if (d) {
        return '';
      } else if (e) {
        return undefined;
      } else if (f) {
        return NaN;
      } else if (g) {
        return 0n;
      } else if (h) {
        return ``;
      } else if (i) {
        return 0.0;
      } else if (j) {
        return 0x0;
      } else if (k) {
        return 0e0;
      }
    },
    falsyReturnsInLoops(items) {
      for (const item of items) {
        if (!item) {
          return false;
        }
      }
      try {
        check(items);
      } catch {
        return false;
      }
    },
  },
};
</script>
