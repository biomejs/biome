<!-- should not generate diagnostics -->
<script setup lang="ts" generic="TData extends object, TRow">
import Table from "./Table.vue";

defineProps<{ rows: TData[] }>();
</script>

<template>
  <Table v-slot="{ row }: { row: TRow }">{{ row }}</Table>
</template>
