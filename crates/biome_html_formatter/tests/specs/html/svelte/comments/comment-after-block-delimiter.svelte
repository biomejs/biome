<span>{#if a}<!-- after if --><i>x</i>{/if}</span>
<span>{#if b}<i>y</i>{:else}<!-- after else --><i>z</i>{/if}</span>
{#each items as it}<!-- after each --><i>{it}</i>{/each}
<span>{#if c}<!-- A --><!-- B --><i>s</i>{/if}</span>
{#if d} <!-- with spaces --> <i>x</i>{/if}
{#if f}<!-- before text -->text{/if}
{#if g}<!-- before expression -->{g}{/if}
{#if h}<!-- before block -->{#if i}<i>x</i>{/if}{/if}
{#if j}<i>x</i>{:else if k}<!-- else if --><i>y</i>{/if}
{#await p}<!-- pending --><i>x</i>{:then v}<!-- then --><i>{v}</i>{:catch e}<!-- catch --><i>{e}</i>{/await}
{#key k}<!-- key --><i>x</i>{/key}
{#snippet s()}<!-- snippet --><i>x</i>{/snippet}
