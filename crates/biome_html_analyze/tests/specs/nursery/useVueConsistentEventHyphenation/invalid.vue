<!-- should generate diagnostics -->
<template>
  <VueComponent @customEvent="onEvent" />
  <VueComponent v-on:customEvent="onEvent" />
  <VueComponent @customEvent.once="onEvent" />
  <VueComponent @update:modelValue="onEvent" />
  <VueComponent @upDate:modelValue="onEvent" />
  <VueComponent @up-date:modelValue="onEvent" />
  <VueComponent @upDate:model-value="onEvent" />
  <VueComponent @CustomEvent="onEvent" />
  <VueComponent @sendHTTPRequest="onEvent" />
  <custom-component @customEvent="onEvent" />
  <Foo.Bar @customEvent="onEvent" />
  <!-- no fix: underscores can't be converted -->
  <VueComponent @custom_Event="onEvent" />
</template>
