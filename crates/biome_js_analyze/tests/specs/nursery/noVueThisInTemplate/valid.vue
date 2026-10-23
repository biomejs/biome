<!-- should not generate diagnostics -->
<script>
export default {
  props: ["foo", "bar", "items"],
  methods: {
    go() {
      return this.foo;
    },
  },
};
</script>

<template>
  <div>{{ foo.bar }}</div>
  <div v-for="foo in bar">{{ foo }}</div>
  <div v-if="foo">{{ foo }}</div>
  <div :class="{ this: foo }">{{ foo }}</div>
  <div :parent="this"></div>
  <div>{{ this.class }}</div>
  <div>{{ this['0'] }}</div>
  <div>{{ this['this'] }}</div>
  <div>{{ this['foo bar'] }}</div>
  <div>{{ this[foo] }}</div>
  <div>{{ this.eval }}</div>
  <div>{{ this.Math }}</div>
  <div v-for="x of xs">{{ this.x }}</div>
  <div v-for="x of xs">{{ this.x() }}</div>
  <div v-for="x of xs">{{ this.x.y() }}</div>
  <div v-for="x of xs">{{ this.x['foo'] }}</div>
  <div v-for="x of xs">{{ this['x'] }}</div>
  <div v-for="(x, index) in xs" :key="this.index"></div>
  <div v-for="x of xs">
    <div v-for="y of x">{{ this.x }} {{ this.y }}</div>
  </div>
  <List v-slot="{ row }">{{ this.row }}</List>
  <List>
    <template #item="{ entry }">{{ this.entry }}</template>
  </List>
  <button @click="(foo) => this.foo = foo"></button>
  <button @click="function () { return this.foo }"></button>
  <button @click="handle(this.$event)"></button>
  <div v-on:[x]="1"></div>
</template>
