<!-- should not generate diagnostics -->
<script>
	import { goto, pushState } from "$app/navigation";
	import { resolve } from "$app/paths";

	goto(resolve("/foo"));
	pushState("", {});
</script>

<a href={resolve("/foo")}>Click me!</a>
