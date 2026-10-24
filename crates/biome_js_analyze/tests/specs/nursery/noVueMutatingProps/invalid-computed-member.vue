<!-- should generate diagnostics -->
<script setup>
const props = defineProps({
  todos: Array,
  matrix: Array,
  lists: Array,
  user: Object,
});

function update(index, row, column, key) {
  props.todos[index].done = true;
  props.matrix[row][column] = 1;
  props.lists[index].push(1);
  props.user[key] = "Alice";
  props["user"].name = "Alice";
}
</script>

<template>
  <input v-model="todos[0].title" />
  <button @click="todos[0].done = true">Done</button>
  <button @click="lists[0].sort()">Sort</button>
</template>
