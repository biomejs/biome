<script>
    // comment in script
    let foo = true;
    let items = [1, 2, 3];
    let promise = Promise.resolve("done!");
    let keyValue = "unique-key";
</script>

<div
    class="foo"
    /* block comment */
>
    text
</div>

<div
    // hello
    attribute="value"
    /* world */
    another-attribute="value"
>
    this works
    <span
        // span comment
        title="nested"
        /* after title */
        data-extra="info"
    >
        Nested span works
    </span>
</div>

<Component
    // hello
    attribute="value"
    /* world */
    another-attribute="value"
>
    this works
    <div
        // nested hello
        class="nested"
        /* another nested */
        id="nested"
    >
        Component with nested div
    </div>
</Component>

{#if true}
    <div
        // hello
        attribute="value"
        /* world */
        another-attribute="value"
    >
        this works
    </div>
{:else}
    <Component
        // hello
        attribute="value"
        /* world */
        another-attribute="value"
    >
        this works
    </Component>
{/if}

{#await promise}
    <div
        // awaiting
        class="promise-pending"
        /* waiting */
        data-state="pending"
    >
        waiting for promise...
    </div>
{:then result}
    <span
        // promise resolved
        class="promise-done"
        /* after class */
        data-state="done"
    >
        Promise resolved: {result}
    </span>
{:catch error}
    <div
        // promise error
        class="promise-error"
        /* error style */
        data-state="error"
    >
        Error: {error}
    </div>
{/await}

{#each items as item, idx (item)}
    <div
        // each block
        class="item-class"
        /* another in each */
        data-index={idx}
    >
        Item: {item}
        <span
            // nested each
            foo="bar"
            /* world */
            baz="qux">Nested in each</span
        >
    </div>
{/each}

{#key keyValue}
    <div
        // key block
        foo="bar"
        /* key comment */
        bar="baz"
    >
        Keyed content: {keyValue}
        <div
            // key-nested
            x="y"
            /* nested key block */
            z="w"
        >
            Nested in key
        </div>
    </div>
{/key}
