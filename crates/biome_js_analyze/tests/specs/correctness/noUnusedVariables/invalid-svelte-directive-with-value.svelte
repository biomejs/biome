<script>
let active = $state(false);
let color = $state("red");
let enabled = $state(true);
</script>

<div class:active={enabled}></div>
<div style:color={"blue"}></div>
