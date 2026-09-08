<!-- should generate diagnostics -->
<div><li /></div>
<ul>{#if visible}<div><li>Indirect child</li></div>{/if}</ul>
{#snippet item()}
    <div><li>Item in a div</li><li /></div>
{/snippet}
