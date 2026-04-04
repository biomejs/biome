{#await       promise      }
     <p>Loading...</p>
{:then     value    }
     <p>The value is {value}</p>
{:catch      error     }
     <p>Error: {error.message}</p>
{/await}
