<form onsubmit={(event)=>{event.preventDefault();submitForm();  trackSubmission();}}></form>

<input bind:value={name}>
<div class={{active: isActive}}></div>
<button on:click={(visible = !visible)}>toggle</button>
<Select options={['a', 'b']} />

<button on:click={handler // the comment stays inside the expression
}>commented</button>

<button on:click={a +}>unformattable</button>
