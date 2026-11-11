<!-- should generate diagnostics -->
<template>
  <div>{{ foo }}</div>
</template>

<script>
import { defineComponent } from "vue";

export const component = defineComponent({
  data() {
    return { foo: "foo" };
  },
});
</script>
