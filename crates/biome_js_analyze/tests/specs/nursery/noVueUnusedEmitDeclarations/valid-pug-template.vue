<!-- should not generate diagnostics -->
<template lang="pug">
button(@click="$emit('foo')") Foo
</template>

<script>
export default {
  emits: ["foo"],
};
</script>
