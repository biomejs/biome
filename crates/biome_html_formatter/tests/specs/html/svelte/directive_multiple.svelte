<input bind:value={name} use:tooltip={{text: 'Enter your name'}} class:invalid={!isValid} />
<div in:fade out:fly={{y: 200}} class:active style:opacity={opacity}>Multiple directives</div>
