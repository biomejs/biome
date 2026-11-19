<!-- should not generate diagnostics -->
<input bind:value />
<input bind:value={name} />
<input bind:value={value.name} />
<input bind:value={() => value, (v) => value = v} />
<div class:active></div>
<div class:active={isActive}></div>
<div class:active={!active}></div>
<div class:is-active={isActive}></div>
<div style:color></div>
<div style:color="red"></div>
<div style:color="{color}px"></div>
<div style:color="{color}{color}"></div>
<div style:--color={color}></div>
<div transition:fade={fade}></div>
<div use:action={action}></div>
<input value={value} />
