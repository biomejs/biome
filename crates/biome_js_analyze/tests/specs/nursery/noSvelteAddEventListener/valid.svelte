<!-- should not generate diagnostics -->
<script>
	import { on } from 'svelte/events';

	const handler = (ev) => {
		console.log(ev);
	};

	function onClick(event) {
		const target = event.currentTarget;
		on(target, 'focus', handler);
	}

	on(window, 'message', handler);
	on(document, 'visibilitychange', handler);

	// Not a call to `addEventListener`.
	window.removeEventListener('message', handler);
	const add = window.addEventListener;
	window['addEventListener']('message', handler);

	// Local function shadowing the global.
	function scoped() {
		function addEventListener() {}
		addEventListener('message', handler);
	}
</script>

<button onclick={onClick}>Hello</button>
