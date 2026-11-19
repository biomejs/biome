<script>
/* should not generate diagnostics */
export default {
  props: {
    a: String,
    b: {
      default(props) {
        return props.a;
      },
    },
    c: {
      default: () => {
        return this.a;
      },
    },
    d: {
      default() {
        return function () {
          return this.a;
        };
      },
    },
    e: {
      type: Object,
      default() {
        return {
          read() {
            return this.a;
          },
          get value() {
            return this.a;
          },
          set value(next = this.a) {
            this.b = next;
          },
        };
      },
    },
    f: {
      default() {
        return class {
          value = this.a;
          static {
            this.b = 1;
          }
          constructor(c = this.a) {
            this.c = c;
          }
          read() {
            return this.a;
          }
          get d() {
            return this.a;
          }
          set d(next) {
            this.b = next;
          }
        };
      },
    },
    printMessage: {
      type: Function,
      default() {
        console.log(this.a);
      },
    },
    g: {
      default: "a",
    },
  },
  methods: {
    read() {
      return this.a;
    },
  },
};
</script>
