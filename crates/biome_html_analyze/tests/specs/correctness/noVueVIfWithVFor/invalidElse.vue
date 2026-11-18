<!-- should generate diagnostics -->
<template>
  <div v-if="foo">foo</div>
  <div v-else v-for="x in xs">{{ x }}</div>

  <div v-if="foo">foo</div>
  <div v-else-if="bar" v-for="x in xs">{{ x }}</div>

  <div v-if="foo">foo</div>
  <div v-for="x in xs" v-else>{{ x }}</div>

  <div v-if="foo">foo</div>
  <TodoItem v-for="todo in todos" v-else-if="bar" :todo="todo" />
</template>
