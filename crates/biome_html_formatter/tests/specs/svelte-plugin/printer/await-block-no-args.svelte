{#await thePromise}
    <p>loading...</p>
{:then}
    <p>the value is {theValue}</p>
{:catch}
    <p>oh no! {theError.message}</p>
{/await}
