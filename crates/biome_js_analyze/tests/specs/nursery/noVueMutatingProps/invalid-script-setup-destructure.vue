<!-- should generate diagnostics -->
<script setup>
const { value, list, user } = defineProps({
  value: String,
  list: Array,
  user: Object,
});

function update() {
  list.push(1);
  user.name = "Alice";
}
</script>

<template>
  <input v-model="value" />
  <input v-model="user.name" />
  <button @click="list.reverse()">Reverse</button>
</template>
