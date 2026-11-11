<!-- should generate diagnostics -->
<template>
  <template v-if="foo" class="heading">...</template>
  <template v-for="i in foo" :bar="i">...</template>
  <template v-slot:foo="foo" ref="input">...</template>
  <template v-if="foo" @click="click">...</template>
  <template v-else-if="bar" id="second">...</template>
  <template v-else v-bind:title="title">...</template>
  <template v-if="foo" v-show="visible">...</template>
  <template v-if="foo" v-memo="[foo]">...</template>
  <template v-if="foo" :[name]="value">...</template>
  <template v-if="foo" v-bind="attrs">...</template>
  <template v-if="foo" v-custom="value">...</template>
  <template v-if="foo" slot="header">...</template>
  <MyList>
    <template #header v-once>...</template>
    <template #item="{ name }" class="item" :data-name="name">{{ name }}</template>
  </MyList>
  <div>
    <template v-for="item in items" :key="item.id" style="color: red">...</template>
  </div>
</template>
