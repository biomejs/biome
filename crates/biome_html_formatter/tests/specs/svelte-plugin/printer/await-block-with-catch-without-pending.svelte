{#await thePromise then theValue}
    <p>the value is {theValue}</p>
{:catch}
    <p>oh no! {theError.message}</p>
{/await}
