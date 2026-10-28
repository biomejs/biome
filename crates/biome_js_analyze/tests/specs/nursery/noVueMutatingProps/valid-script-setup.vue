<!-- should not generate diagnostics -->
<script setup>
import { computed, ref } from "vue";

const props = defineProps({
  count: Number,
  items: Array,
});
const emit = defineEmits(["update:count"]);
const model = defineModel();

const localCount = ref(props.count);
const sortedItems = computed(() => [...props.items].sort());

function increment(index) {
  console.log(props.items[index].name);
  const copy = Object.assign({}, props.items[index]);
  sortedItems.value[index].done = true;
  localCount.value++;
  model.value = "test";
  emit("update:count", props.count + 1);
}
</script>

<template>
  <input v-model="model" />
  <input v-model="localCount" />
  <span>{{ count }} {{ sortedItems.length }}</span>
</template>
