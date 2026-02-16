{#if visible}
  <div in:fly={{ y: 200 }} out:fade>
    flies in, fades out
  </div>
  <div in:fly|global={{ y: 200 }} out:fade|local={{ duration: 300 }}>
    With modifiers
  </div>
{/if}
