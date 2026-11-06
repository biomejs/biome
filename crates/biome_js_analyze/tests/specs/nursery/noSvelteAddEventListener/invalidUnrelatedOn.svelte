<!-- should generate diagnostics -->
<script>
	// `on` is not the one from `svelte/events`: no fix.
	import { on } from 'some-library';
	import { off as on2 } from 'svelte/events';

	window.addEventListener('message', handler);

	function shadowed() {
		const window = {};
		// The fix would reference the local `window`: no fix.
		addEventListener('message', handler);
	}
</script>
