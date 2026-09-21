<!-- should not generate diagnostics -->
<template>
  <div>
    <template #default="{ value }">{{ value }}</template>
    <template #trigger="{ open, toggle }">
      <button :aria-expanded="open" type="button" @click="toggle">Toggle</button>
    </template>
    <template v-slot="props">{{ props.value }}</template>
    <template v-slot:header="{ title: heading, nested: { label }, ...rest }">
      {{ heading }} {{ label }} {{ rest }}
    </template>
    <template #items="[first, , ...tail]">{{ first }} {{ tail }}</template>
  </div>
</template>
<script lang="ts" setup></script>
