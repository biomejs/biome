<!-- should generate diagnostics -->
<template>
  <VueComponent @custom-event="onEvent" />
  <VueComponent v-on:custom-event="onEvent" />
  <VueComponent @update:model-value="onEvent" />
  <VueComponent @up-date:modelValue="onEvent" />
  <VueComponent @upDate:model-value="onEvent" />
  <VueComponent @up-date:model-value="onEvent" />
  <custom-component @custom-event="onEvent" />
  <VueComponent @custom--event="onEvent" />
</template>
