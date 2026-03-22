<input dir="auto" spellcheck="true" tabindex="-1" type="checkbox" bind:checked={accepted} />
<input dir="auto" spellcheck="true" tabindex="-1" type="checkbox" bind:checked defaultChecked={true}>
<div style:color="red" class="mydiv" dir="auto" spellcheck="true" tabindex="-1" >some text</div>

<div
	use:gestures
    transition:fade
	onswipeleft={next}
	onswiperight={prev}
    {@attach myAttachment}
    dir="auto"
    spellcheck="true"
    tabindex="-1"
    animate:flip
    style:color="red"
>...</div>
