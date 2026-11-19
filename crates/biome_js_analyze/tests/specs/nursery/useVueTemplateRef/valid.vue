<!-- should not generate diagnostics -->
<template>
  <div ref="root" />
  <button ref="button">Content</button>
  <a href="" ref="link">Link</a>
  <ul>
    <li v-for="food in foods" :key="food">{{ food }}</li>
    <li ref="firstListItem">Morning</li>
    <li ref="second">Afternoon</li>
  </ul>
  <input :ref="(el) => (input = el)" />
  <span :ref="label" />
</template>

<script setup>
import { ref, shallowRef, useTemplateRef } from "vue";

const root = useTemplateRef("root");
const buttonRef = useTemplateRef("button");
const link = useTemplateRef("link");
const foods = ref(["Spaghetti", "Pizza", "Cake"]);
const input = ref();
const label = ref();

function getFirstListItemElement() {
  const firstListItem = ref();
  const nestedCallback = () => {
    const second = shallowRef();
    console.log(second);
  };
  nestedCallback();
}
</script>
