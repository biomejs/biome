<script>
    export let thePromise;
</script>

{#await thePromise}
    loading...
{:then { result: r, error: e }}
    <p>error: {e}</p>
    <p>result: {r}</p>
{:catch { error: { message: m, code: c } }}
    <p>message: {m}</p>
    <p>code: {c}</p>
{/await}
