<!-- should generate diagnostics -->

<TodoItem
  v-if="complete"
  v-for="todo in todos"
  :todo="todo"
/>

<TodoItem
  v-for="todo in todos"
  v-if="todo.shown"
  :todo="todo"
/>
