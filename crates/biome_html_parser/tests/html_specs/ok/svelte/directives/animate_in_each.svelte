{#each todos.filter(t => !t.done) as todo (todo.id)}
  <div animate:flip={{ duration: 200 }}>
    <input type="checkbox" bind:checked={todo.done} />
    {todo.text}
  </div>
{/each}
