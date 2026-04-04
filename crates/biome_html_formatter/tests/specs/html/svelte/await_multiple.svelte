{#await userPromise then user}
  <h1>{user.name}</h1>
{/await}

{#await postsPromise then posts}
  <ul>
    {#each posts as post}
      <li>{post.title}</li>
    {/each}
  </ul>
{/await}

{#await commentsPromise}
  <p>Loading comments...</p>
{:then comments}
  <div>
    {#each comments as comment}
      <p>{comment.text}</p>
    {/each}
  </div>
{/await}
