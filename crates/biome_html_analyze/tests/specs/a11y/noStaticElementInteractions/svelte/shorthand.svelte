<!-- should generate diagnostics -->
<script>
    let { onclick, click, onClick } = $props();
</script>

<div { onclick }></div>
<div onclickcapture={onclick}></div>
<button {onclick}>Run</button>
<div {click}></div>
<div click={click}></div>
<div {onClick}></div>
