<script>
/* should generate diagnostics */
export default {
  props: {
    a: String,
    b: {
      default() {
        return this.a;
      },
    },
    c: {
      type: Array,
      default: function () {
        return [this.a];
      },
    },
    d: {
      default() {
        return () => this.a;
      },
    },
    e: {
      type: Object,
      default: (function () {
        return { a: this.a };
      }),
    },
    f: {
      default(value = this.a) {
        return value;
      },
    },
    g: {
      type: [String, Number],
      default() {
        if (this.a) {
          return this.a;
        }
        return 0;
      },
    },
    h: {
      type: Object,
      default() {
        return {
          [this.a]() {},
        };
      },
    },
    i: {
      default() {
        return class extends this.a {};
      },
    },
    j: {
      type: [Function, String],
      default() {
        return this.a;
      },
    },
  },
};
</script>
