{#snippet foo()}
    <p>foo</p>
{/snippet}

{#snippet bar({ a, b })}
    <p>bar</p>
{/snippet}

{#snippet baz(a, b, c = 1)}
    <p>baz</p>
{/snippet}

<div>
    {#snippet loooongFunction(
        a,
        lot,
        _of,
        parameters,
        that,
        make,
        the,
        lines,
        _break,
    )}
        <p>baz</p>
    {/snippet}
</div>

{@render foo()}
{@render foo?.()}
{@render bar(x)}
{@render bar.baz[buzz](x)}
{@render (why ? not : like_this)(x)}
{@render test((() => "a")())}
{@render test(t())}
