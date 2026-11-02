<!-- should not generate diagnostics -->
<template>
  <div v-for="i in 5">
    <div v-for="i in 5"></div>
  </div>
  <MyList v-slot="{ i }">{{ i }}</MyList>
</template>

<script setup>
defineProps({ i: Number });
</script>
