<!-- should generate diagnostics -->
<template>
  <template #named="{ source: alias, nested: { label } }">
    {{ alias }} {{ label }}
    {{ source }} {{ nested }} {{ missing }}
  </template>
  <div v-bind="{ ordinaryBinding }">{{ ordinaryBinding }}</div>
</template>
