<!-- should generate diagnostics -->
<script>
	import { on } from 'svelte/events';

	const handler = (ev) => {
		console.log(ev);
	};

	function onClick(event) {
		const target = event.currentTarget;
		const deepObj = { deep: { obj: { target } } };

		target.addEventListener('focus', handler);
		deepObj.deep.obj.target.addEventListener('focus', handler);
	}

	addEventListener('message', handler);
	window.addEventListener('message', handler);
	document.addEventListener('visibilitychange', handler);
	window.addEventListener    ('message', handler);
	window.addEventListener/* foo */('message', handler);
	window.addEventListener('message', handler, { once: true },);
	window.addEventListener(...params);
	window.addEventListener();
	(a, b).addEventListener('message', handler);
	(window).addEventListener('message', handler);

	// No fix: short-circuits when `window` is nullish.
	window?.addEventListener('message', handler);
</script>
