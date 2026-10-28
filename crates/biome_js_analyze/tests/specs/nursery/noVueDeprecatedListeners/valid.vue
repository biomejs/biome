<!-- should not generate diagnostics -->
<template>
  <div v-bind="$attrs" />
  <div foo="$listeners" />
</template>

<script>
export default {
  mounted() {
    this.$emit("start");
    this["$listeners"];
    this.$attrs;
    other.$listeners;

    const { vm } = this;
    vm.$listeners;

    let mutableAlias = this;
    mutableAlias.$listeners;

    const indirectAlias = this;
    const anotherAlias = indirectAlias;
    anotherAlias.$listeners;
  },
  methods: {
    click($listeners) {
      foo.$listeners;
    },
  },
};

const another = function () {
  console.log(this.$listeners);
};
</script>
