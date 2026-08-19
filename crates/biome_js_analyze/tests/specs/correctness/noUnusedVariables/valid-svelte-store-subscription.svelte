<!-- should not generate diagnostics -->
<!-- Regression test: $errors/$form in the template should suppress the -->
<!-- "unused variable" diagnostic for the destructured `errors`/`form`  -->
<!-- bindings in the script, even when $-prefixed references only appear  -->
<!-- in the HTML template (not in the script block itself).              -->
<script lang="ts">
	import { superForm } from 'sveltekit-superforms';

	const { form, errors, enhance } = superForm();
</script>

<form use:enhance>
	<input bind:value={$form.email} />
	<span>{$errors.email}</span>
</form>
