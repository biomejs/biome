<script>
  let user = { name: 'Alice', age: 30, isActive: true };
  let items = [
    { id: 1, name: 'Item 1', category: 'A', tags: ['red', 'blue'] },
    { id: 2, name: 'Item 2', category: 'B', tags: ['green'] },
    { id: 3, name: 'Item 3', category: 'A', tags: ['red', 'yellow'] },
    { id: 4, name: 'Item 4', category: 'C', tags: ['blue'] },
  ];
  let promise = fetch('/api/data');
  let key = 0;
</script>

{#if user.isActive}
  <div class="user-active">
    <h1>Welcome, {user.name}!</h1>
    {#if user.age >= 18}
      <p>You are an adult.</p>
      {#if user.age >= 65}
        <span>Senior discount available</span>
      {:else if user.age >= 21}
        <span>Full access granted</span>
      {:else}
        <span>Limited access</span>
      {/if}
    {:else}
      <p>You are a minor.</p>
    {/if}
  </div>
{:else}
  <p>User is inactive</p>
{/if}

{#each items as item, index (item.id)}
  <div class="item" data-index={index}>
    <h3>{item.name}</h3>
    {#if item.category === 'A'}
      <span class="badge badge-a">Category A</span>
    {:else if item.category === 'B'}
      <span class="badge badge-b">Category B</span>
    {:else}
      <span class="badge badge-other">Other</span>
    {/if}
    <ul>
      {#each item.tags as tag, tagIndex}
        <li class="tag" data-idx={tagIndex}>{tag}</li>
      {/each}
    </ul>
  </div>
{:else}
  <p>No items found</p>
{/each}

{#await promise}
  <div class="loading">
    <span>Loading data...</span>
    {#if true}
      <progress>Please wait</progress>
    {/if}
  </div>
{:then data}
  <div class="success">
    <h2>Data loaded!</h2>
    {#if data.items}
      {#each data.items as subItem}
        <p>{subItem.value}</p>
      {/each}
    {/if}
  </div>
{:catch error}
  <div class="error">
    <p>Failed to load: {error.message}</p>
    {#if error.code === 404}
      <span>Resource not found</span>
    {:else}
      <span>Unknown error occurred</span>
    {/if}
  </div>
{/await}

{#key key}
  <div class="keyed-content">
    <p>This content re-renders when key changes: {key}</p>
    {#if key > 0}
      <span>Key is positive</span>
    {/if}
    {#each items.slice(0, key) as item}
      <div>{item.name}</div>
    {/each}
  </div>
{/key}

{#if items.length > 0}
  {#each items as outerItem}
    {#if outerItem.tags.length > 0}
      {#each outerItem.tags as innerTag}
        {#if innerTag.length > 3}
          <span class="long-tag">{innerTag}</span>
        {/if}
      {/each}
    {/if}
  {/each}
{/if}

{#await fetch('/api/user') then userData}
  {#if userData.profile}
    {#each userData.profile.settings as setting}
      <div>{setting.name}: {setting.value}</div>
    {/each}
  {/if}
{/await}
