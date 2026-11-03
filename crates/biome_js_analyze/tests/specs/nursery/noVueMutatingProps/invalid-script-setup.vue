<!-- should generate diagnostics -->
<script setup>
const props = defineProps({
  value: String,
  list: Array,
  user: Object,
});

function update() {
  props.value = "test";
  props.value++;
  props.list.push(1);
  props.user.name = "Alice";
  delete props.user.name;
  Object.assign(props.user, { name: "Alice" });
}
</script>

<template>
  <input v-model="props.value" />
  <input v-model="value" />
  <button @click="props.list.pop()">Pop</button>
  <button @click="user.name = 'Bob'">Rename</button>
</template>
