<script>
	import { goto, pushState } from "$app/navigation";
	import { resolve } from "$app/paths";

	goto("/foo");
	pushState("/foo", {});

	function onclick() {
		goto(resolve("/foo") + "/bar");
	}
</script>

<button {onclick}>Click me!</button>
