<!-- should not generate diagnostics -->
<template>
  <button @click="$emit('close')">Close</button>
</template>

<script>
import { helper } from "./helper";
helper();
</script>
