<!-- should not generate diagnostics -->
<template>
  <label ref="labelElem">
    Name:
    <input v-model="name" />
  </label>
  <button ref="button">Click</button>
</template>

<script>
import { ref, useTemplateRef } from "vue";

export default {
  name: "NameRow",
  setup() {
    const counter = ref(0);
    const button = useTemplateRef("button");
    return { counter, button };
  },
  data() {
    return {
      label: ref("label"),
      labelElem: ref(),
      name: "",
    };
  },
};
</script>
