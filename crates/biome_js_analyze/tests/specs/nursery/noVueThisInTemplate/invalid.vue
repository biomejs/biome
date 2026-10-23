<!-- should generate diagnostics -->
<script>
export default {
  props: ["foo", "bar", "items"],
  data() {
    return { count: 0, xs: [], xs0AZ_foo: 1 };
  },
};
</script>

<template>
  <div>{{ this.foo }}</div>
  <div>{{ this.foo() }}</div>
  <div>{{ this.foo.bar() }}</div>
  <div :class="this.foo"></div>
  <div :class="{ foo: this.foo }"></div>
  <div :class="{ foo: this.foo() }"></div>
  <div v-if="this.foo"></div>
  <div v-for="foo in this.bar"></div>
  <div>{{ this?.foo }}</div>
  <div>{{ this?.foo.bar }}</div>
  <div>{{ this['xs'] }}</div>
  <div>{{ this["xs0AZ_foo"] }}</div>
  <div>{{ (this).foo }}</div>
  <div v-if="fn(this.$foo)"></div>
  <div :class="{ foo: this.$foo }"></div>
  <button @click="this.$emit('change')"></button>
  <button @click="this.count++"></button>
  <button @click="this.count = 1"></button>
  <button @click="this['count'] += 1"></button>
  <button @click="() => this.count++"></button>
  <button @click="handle($event, this.count)"></button>
  <div v-for="item in items">{{ this.count }} {{ item }}</div>
  <List v-slot="{ row }">{{ this.count }} {{ row }}</List>
  <List v-for="item in items" v-slot="{ row }" :row="this.row"></List>
  <div v-for="item in items" v-if="this.item"></div>
  <div>{{ /* before */ this.foo /* after */ }}</div>
</template>
