<!-- should not generate diagnostics -->
<template>

<ul v-if="complete">
  <TodoItem
    v-for="todo in todos"
    :todo="todo"
  />
</ul>

<TodoItem
  v-for="todo in shownTodos"
  :todo="todo"
/>
</template>
