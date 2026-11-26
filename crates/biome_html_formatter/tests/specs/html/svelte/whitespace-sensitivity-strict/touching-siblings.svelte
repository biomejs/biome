{#snippet parts()}{#if leftIcon}<span class="icon">{@render leftIcon()}</span>{/if}{@render children?.()}{#if rightIcon}<span class="icon">{@render rightIcon()}</span>{/if}{/snippet}

<p><span class="label">{label}</span><Input value={label} /><span class="hint">{hint}</span>{#if showAddons}<span>addons</span>{/if}</p>