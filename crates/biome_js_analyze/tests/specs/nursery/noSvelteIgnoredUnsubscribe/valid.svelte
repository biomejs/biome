<!-- should not generate diagnostics -->
<script>
	import { onDestroy } from "svelte";
	import { writable } from "svelte/store";

	const foo = writable(0);

	const unsubscribe = foo.subscribe(() => {
		console.log("foo changed");
	});
	onDestroy(unsubscribe);

	const unsubscribers = [];
	unsubscribers.push(
		foo.subscribe(() => {
			console.log("foo changed");
		})
	);

	let stop;
	stop = foo.subscribe(() => {});

	const subscriptions = {};
	subscriptions.foo = foo.subscribe(() => {});

	onDestroy(foo.subscribe(() => {}));

	a(foo.subscribe);
	foo.subscribe;
	subscribe(() => {});
	foo.subscribeAll(() => {});
	void foo.subscribe(() => {});

	function track() {
		return foo.subscribe(() => {});
	}

	$effect(() => foo.subscribe(() => {}));
</script>

<p>{$foo}</p>
