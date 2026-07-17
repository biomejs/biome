<input type="checkbox" spellcheck="true" tabindex="-1" dir="auto" bind:checked={accepted} />
<input type="checkbox" spellcheck="true" tabindex="-1" dir="auto" bind:checked defaultChecked={true}>
<div style:color="red" spellcheck="true" tabindex="-1" dir="auto" class="mydiv">some text</div>

<img use:myaction={data} spellcheck="true" tabindex="-1" dir="auto" {src} alt="some alt text" />

<div
    bind:value2={a}
    bind:value1={a}
    use:myAction
	use:gestures
    out:fade
    transition:fade
    style:width="12rem"
	onswipeleft={next}
	onswiperight={prev}
    {@attach myAttachment}
    spellcheck="true"
    in:fly={{ y: 200 }}
    tabindex="-1"
    dir="auto"
    class:cool={cool}
    {foo}
    class:myClass={foo}
    animate:whizz
    bind:this={canvas}
    transition:fly={{ y: 200, duration: 2000 }}
    animate:flip
    style:color="red"
>...</div>

<div
    bind:value2={a}
    bind:value1={a}
    use:myAction
	use:gestures
    out:fade
    transition:fade
    style:width="12rem"
	onswipeleft={next}
	onswiperight={prev}
    {@attach myAttachment}
    {...props}
    spellcheck="true"
    in:fly={{ y: 200 }}
    tabindex="-1"
    dir="auto"
    class:cool={cool}
    {foo}
    class:myClass={foo}
    animate:whizz
    bind:this={canvas}
    transition:fly={{ y: 200, duration: 2000 }}
    animate:flip
    style:color="red"
>...</div>
