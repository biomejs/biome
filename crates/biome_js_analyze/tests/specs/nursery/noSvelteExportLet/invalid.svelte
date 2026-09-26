<!-- should generate diagnostics -->
<script>
	export let name;
	export let count = 0, label = "label";
	export var legacy;

	let className = "";
	export { className as class };

	let shorthand;
	export { shorthand };

	let { first } = someObject;
	export { first };
</script>

<p class={className}>{name} {count} {label} {legacy} {shorthand} {first}</p>
