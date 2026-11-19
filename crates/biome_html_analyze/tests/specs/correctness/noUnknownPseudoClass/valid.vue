<!-- should not generate diagnostics -->

<template>
  <button>
    <span>123</span>
  </button>
</template>

<script lang="ts" setup>

</script>

<style>
button {
  height: 100px;

  & :deep(span) {
    color: #fff
  }
}
</style>
