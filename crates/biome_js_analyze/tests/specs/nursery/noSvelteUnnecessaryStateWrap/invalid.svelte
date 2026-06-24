<!-- should generate diagnostics -->
<script>
	import { SvelteMap, SvelteSet, SvelteDate, SvelteURL, SvelteURLSearchParams } from "svelte/reactivity";
	import { MediaQuery } from "svelte/reactivity";

	// All built-in reactive classes wrapped unnecessarily.
	const map = $state(new SvelteMap());
	const set = $state(new SvelteSet());
	const date = $state(new SvelteDate());
	const url = $state(new SvelteURL("https://example.com"));
	const params = $state(new SvelteURLSearchParams());
	const mq = $state(new MediaQuery("(min-width: 768px)"));
</script>
