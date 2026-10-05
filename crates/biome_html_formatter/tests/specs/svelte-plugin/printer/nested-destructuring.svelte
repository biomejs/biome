{#await thePromise}
    <p>loading...</p>
{:then { data: [theValue] }}
    <p>the value is {theValue}</p>
{:catch { data: [{ data: [theError] }] }}
    <p>oh no! {theError.message}</p>
{/await}
