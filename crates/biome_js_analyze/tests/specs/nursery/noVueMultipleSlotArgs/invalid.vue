<!-- should generate diagnostics -->
<script>
export default {
  render() {
    this.$slots.default(foo, bar);
    this.$slots.foo(foo, { bar });
    this.$slots["header"](foo, bar);
    this.$slots.default(...foo);
    this.$slots.default(...foo, bar);

    this?.$slots?.default?.(foo, bar);
    this.$slots.default?.(foo, bar);
    (this?.$slots)?.default?.(foo, bar);
    (this.$slots.default)(foo, bar);

    const vm = this;
    vm.$slots.default(foo, bar);
    vm?.$slots?.foo?.(foo, bar);

    const item = this.$slots.item;
    item(foo, bar);

    let children;
    children = this.$slots.children;
    if (children) children(...foo);
  },
};
</script>
