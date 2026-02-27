{#await userPromise}
  <p>Loading user...</p>
{:then user}
  <div>
    <h1>{user.name}</h1>
    {#await user.postsPromise then posts}
      <ul>
        {#each posts as post}
          <li>{post.title}</li>
        {/each}
      </ul>
    {/await}
  </div>
{:catch error}
  <p>Error: {error.message}</p>
{/await}
