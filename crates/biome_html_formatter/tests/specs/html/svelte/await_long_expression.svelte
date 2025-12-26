{#await fetch('/api/users/' + userId).then(res => res.json()).then(data => data.profile).catch(err => handleError(err))}
  <p>Loading...</p>
{:then profile}
  <div>{profile.name}</div>
{/await}
