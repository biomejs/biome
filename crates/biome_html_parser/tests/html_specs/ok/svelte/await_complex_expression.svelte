{#await fetchUser(userId).then(u => u.profile)}
  <p>Loading user profile...</p>
{:then profile}
  <div>
    <h1>{profile.name}</h1>
    <p>{profile.bio}</p>
  </div>
{:catch error}
  <p>Failed to load profile: {error.message}</p>
{/await}
