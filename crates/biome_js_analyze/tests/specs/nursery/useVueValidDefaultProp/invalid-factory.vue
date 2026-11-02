<script>
/* should generate diagnostics */
export default {
  props: {
    a: {
      type: Number,
      default() {
        return "";
      },
    },
    b: {
      type: Object,
      default() {
        return "";
      },
    },
    c: {
      type: Number,
      default: () => {
        return "";
      },
    },
    d: { type: Number, default: () => "" },
    e: { type: Object, default: () => "" },
    f: { type: String, default: function () { return 123; } },
    g: { type: Object, default: () => () => ({}) },
    h: {
      type: [String, Boolean],
      default() {
        switch (kind) {
          case 1: return 1;
          case 2: return "";
          case 3: return {};
          case 4: return Foo;
          case 5: return () => {};
          case 6: return false;
        }
        function nested() {
          return 1;
        }
      },
    },
  },
};
</script>
