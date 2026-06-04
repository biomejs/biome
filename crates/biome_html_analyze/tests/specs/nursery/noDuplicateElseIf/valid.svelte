<!-- should not generate diagnostics -->

{#if a}
  <div>a</div>
{:else if b}
  <div>b</div>
{:else if c}
  <div>c</div>
{/if}

{#if x === 1}
  <div>one</div>
{:else if x === 2}
  <div>two</div>
{:else if x === 3}
  <div>three</div>
{/if}

{#if foo}
  <div>foo</div>
{:else}
  <div>not foo</div>
{/if}

{#if a}
  <div>a</div>
{/if}
