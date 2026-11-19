<!-- should generate diagnostics -->

<script>
	let { active, isActive, props } = $props();
</script>

<div class:active={isActive}></div>
<div class:active></div>
<div class="button" class:active={isActive}></div>
<div class:active class:inactive={!active}></div>
<div class:my-class={isActive}></div>
<div class:active="{isActive}"></div>
<div
	class="button"
	class:active={isActive}
></div>
<svelte:element this="div" class:active={isActive}></svelte:element>
<button {...props} class:active></button>
