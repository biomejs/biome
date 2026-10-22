<!-- should generate diagnostics -->
<script>
import { h } from "vue";

export default {
  render() {
    var children = this.$slots.default;
    var filtered = this.$slots.default.filter(test);
    return h("div", [...children, filtered]);
  },
  methods: {
    members() {
      this.$slots.foo.length;
      this.$slots.foo[0];
      this.$slots["foo"].length;
      this["$slots"].foo.length;
      this.$slots[name].length;
    },
    optionalChains() {
      this.$slots.foo?.bar;
      (this.$slots?.foo)?.bar;
      (this?.$slots)?.foo?.bar;
    },
    wrappers() {
      (this.$slots.foo).length;
      (0, this.$slots.foo).length;
    },
    arrays() {
      return [this.$slots.foo, ...this.$slots.bar];
    },
    spreadArguments() {
      return Math.max(...this.$slots.foo);
    },
    destructuring() {
      const [first] = this.$slots.foo;
      const { length } = this.$slots.bar;
      return [first, length];
    },
    assignedVariable() {
      let children;
      children = this.$slots.foo;
      return children.length;
    },
    chainedVariables() {
      const first = this.$slots.foo;
      const second = first;
      return second.length;
    },
    writes() {
      this.$slots.foo.bar = value;
      this.$slots.foo[0] = value;
      this.$slots.foo.count++;
      delete this.$slots.foo.bar;
      const children = this.$slots.bar;
      children.length = 0;
    },
    thisAlias() {
      const vm = this;
      return vm.$slots.foo.length;
    },
  },
};
</script>
