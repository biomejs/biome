<!-- should not generate diagnostics -->
<template>
  <div v-if="$slots.default" />
</template>

<script>
export default {
  render() {
    this["$scopedSlots"];
    this.$slots;
    other.$scopedSlots;

    let mutableAlias = this;
    mutableAlias.$scopedSlots;

    const indirectAlias = this;
    const anotherAlias = indirectAlias;
    anotherAlias.$scopedSlots;
  },
};

function outsideComponent() {
  return this.$scopedSlots;
}
</script>
