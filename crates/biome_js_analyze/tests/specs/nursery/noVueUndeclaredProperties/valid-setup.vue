<!-- should not generate diagnostics -->
<script setup>
import { computed, ref } from "vue";
import Child from "./Child.vue";

const props = defineProps(["name"]);
const model = defineModel();
const count = ref(0);
const doubled = computed(() => count.value * 2);
function increment() {
  count.value++;
}
const { title = "Untitled" } = defineProps(["title"]);
</script>

<template>
  <Child :name="name" :model="model" @click="increment">
    {{ count }} {{ doubled }} {{ props.name }} {{ $props.name }} {{ title }}
  </Child>
</template>
