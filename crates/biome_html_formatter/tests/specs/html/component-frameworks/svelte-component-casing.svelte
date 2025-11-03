<script lang="ts">
  import Button from './Button.svelte';
  import TextInput from './TextInput.svelte';
  import Select from './Select.svelte';
</script>

<Button label="test button" />
<TextInput placeholder="enter text" />
<Select options={['a', 'b']} />
<button>native button</button>
<input type="text" />
<select><option>native</option></select>
