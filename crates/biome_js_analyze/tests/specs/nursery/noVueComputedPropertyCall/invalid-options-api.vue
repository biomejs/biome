<!-- should generate diagnostics -->
<script>
export default {
  props: {
    label: String,
    count: { type: Number },
    flag: [Boolean, String],
  },
  data() {
    return { text: "hello", items: [1, 2] };
  },
  computed: {
    fromData() {
      return this.text;
    },
    fromProp() {
      return this.label;
    },
    fromOptionsProp() {
      return this.count;
    },
    fromPropTypes() {
      return this.flag;
    },
    literal() {
      return "literal";
    },
    fromComputed() {
      return this.literal;
    },
    object() {
      return { inside() {} };
    },
    empty() {
      return;
    },
    arrowGetter: {
      get: () => 42,
    },
    branches() {
      return this.label ? this.fromMethod() : "fallback";
    },
    usesOther() {
      return this.literal();
    },
    list() {
      return this.items;
    },
  },
  methods: {
    fromMethod() {
      return 1;
    },
    run() {
      this.fromData();
      this.fromProp();
      this.fromOptionsProp();
      this.fromPropTypes();
      this.literal();
      this.fromComputed();
      this.object().inside();
      this.empty();
      this.arrowGetter();
      this.branches();
      (this.literal)();
      this.list.map(() => [...this.list(), ...this.object()]);
    },
  },
};
</script>
