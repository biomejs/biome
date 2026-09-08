<!-- should generate diagnostics -->
<template>
  <div v-if="$scopedSlots.default" />
  <p>{{ $scopedSlots.default }}</p>
  <button @click="$scopedSlots = replacement" />
  <div :class="{ $scopedSlots }" />
</template>

<script>
export default {
  render() {
    this.$scopedSlots;

    const vm = this;
    vm.$scopedSlots = {};
  },
};
</script>
