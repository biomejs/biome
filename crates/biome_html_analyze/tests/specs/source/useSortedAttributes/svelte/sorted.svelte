<input dir="auto" spellcheck="true" tabindex="-1" type="checkbox" bind:checked={accepted} />
<input defaultChecked={true} dir="auto" spellcheck="true" tabindex="-1" type="checkbox" bind:checked>
<div class="mydiv" dir="auto" spellcheck="true" tabindex="-1" style:color="red">some text</div>

<img alt="some alt text" dir="auto" spellcheck="true" {src} tabindex="-1" use:myaction={data} />

<div
    dir="auto"
    {foo}
    onswipeleft={next}
	onswiperight={prev}
    spellcheck="true"
    tabindex="-1"
    bind:this={canvas}
    style:color="red"
    style:width="12rem"
    class:cool={cool}
    class:myClass={foo}
    bind:value1={a}
    bind:value2={a}
	use:gestures
    use:myAction
    transition:fade
    transition:fly={{ y: 200, duration: 2000 }}
    in:fly={{ y: 200 }}
    out:fade
    animate:flip
    animate:whizz
    {@attach myAttachment}
>...</div>

<div
	onswipeleft={next}
	onswiperight={prev}
    style:width="12rem"
    bind:value1={a}
    bind:value2={a}
	use:gestures
    use:myAction
    transition:fade
    out:fade
    {@attach myAttachment}
    {...props}
    dir="auto"
    {foo}
    spellcheck="true"
    tabindex="-1"
    bind:this={canvas}
    style:color="red"
    class:cool={cool}
    class:myClass={foo}
    transition:fly={{ y: 200, duration: 2000 }}
    in:fly={{ y: 200 }}
    animate:flip
    animate:whizz
>...</div>
