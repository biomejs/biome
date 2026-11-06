<!-- should not generate diagnostics -->
<script setup lang="ts" generic="TData extends object, const TKey extends keyof TData = keyof TData">
import Table from "./Table.vue";

defineProps<{ rows: TData[]; column: TKey }>();
</script>

<template>
  <Table v-slot="{ row }: { row: TData }">{{ row }}</Table>
</template>
