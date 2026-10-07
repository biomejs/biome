<form
  class="mt-4 flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto p-1"
  onsubmit={(e) => {
    // Keep the form on this page.
       e.preventDefault();
        abc(); save(); xyz();
  }}
>
  <button class:active={ ready   && running } on:click={(event)=>{event.preventDefault(); run()}}>Save</button>
</form>

<Component value={(value)} bind:value={value} use:tooltip={{text:'Save'}} transition:fade={{duration: 200, delay:100}} />
<input bind:value={() =>value, (next)=>{value=next;notify()}} />
<div title={`line one\nline two`}>{  text }</div>
<div style:color={palette . primary} in:fade={{duration:  250}} out:fade={{duration: 125}} />
