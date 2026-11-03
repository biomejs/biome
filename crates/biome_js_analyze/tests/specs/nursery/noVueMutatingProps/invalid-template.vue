<!-- should generate diagnostics -->
<script>
export default {
  props: ["value", "list", "user", "count"],
};
</script>

<template>
  <input v-model="value" />
  <input v-model="user.name" />
  <button @click="value = 'test'">Assign</button>
  <button @click="list.push(1)">Push</button>
  <button @click="user.id++">Increment member</button>
  <button @click="count++">Increment</button>
  <button @click="Object.assign(user, { id: 1 })">Assign object</button>
  <button @click="$props.value = 'test'">Assign through $props</button>
</template>
