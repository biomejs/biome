<script>
/* should generate diagnostics */
export default {
  computed: {
    empty() {},
    functionExpression: function () {},
    noElse() {
      if (this.ready) {
        return this.value;
      }
    },
    emptyReturn() {
      return;
    },
    emptyReturnInBranch() {
      if (this.ready) {
        return;
      }
      return this.value;
    },
    accessors: {
      set(value) {
        this.value = value;
      },
      get() {},
    },
    arrowAccessor: {
      get: () => {},
    },
    innerFunctionReturns() {
      function double(value) {
        return value * 2;
      }
      double(this.value);
    },
    switchWithoutDefault() {
      switch (this.kind) {
        case "a":
          return 1;
        case "b":
          return 2;
      }
    },
    loopWithBreak() {
      for (const item of this.items) {
        if (item.ready) {
          break;
        }
        return item;
      }
    },
    catchWithoutReturn() {
      try {
        return JSON.parse(this.raw);
      } catch {
        console.error("invalid JSON");
      }
    },
  },
};
</script>
