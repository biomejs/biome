<!-- should generate diagnostics -->

{#if a}
  <div>a</div>
{:else if b}
  <div>b</div>
{:else if a}
  <div>a again</div>
{/if}

{#if x === 1}
  <div>one</div>
{:else if x === 2}
  <div>two</div>
{:else if x === 1}
  <div>one again</div>
{/if}

{#if foo}
  <div>foo</div>
{:else if bar}
  <div>bar</div>
{:else if foo}
  <div>foo again</div>
{:else if baz}
  <div>baz</div>
{/if}
