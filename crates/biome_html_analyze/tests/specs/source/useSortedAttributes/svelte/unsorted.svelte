<input type="checkbox" spellcheck="true" tabindex="-1" dir="auto" bind:checked={accepted} />
<input type="checkbox" spellcheck="true" tabindex="-1" dir="auto" bind:checked defaultChecked={true}>
<div style:color="red" spellcheck="true" tabindex="-1" dir="auto" class="mydiv">some text</div>

<div
	use:gestures
    transition:fade
	onswipeleft={next}
	onswiperight={prev}
    {@attach myAttachment}
    spellcheck="true"
    tabindex="-1"
    dir="auto"
    animate:flip
    style:color="red"
>...</div>
