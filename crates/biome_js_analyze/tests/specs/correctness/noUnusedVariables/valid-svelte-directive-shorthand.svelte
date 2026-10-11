<!-- should not generate diagnostics -->
<script>
let active = $state(false);
let color = $state("red");
let invalid = $state(false);
let size = $state("1rem");
</script>

<div class:active></div>
<div style:color></div>
<input class:invalid/>
<div style:size|important></div>
