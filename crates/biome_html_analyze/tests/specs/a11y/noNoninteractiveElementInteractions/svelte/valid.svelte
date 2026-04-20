<!-- should not generate diagnostics -->
<TestComponent onclick={doFoo} />
<Button onclick={doFoo} />
<Image onclick={() => void 0} />

<!-- Input element -->
<input onclick={() => void 0} />
<input type="button" onclick={() => void 0} />
<input type="checkbox" onclick={() => void 0} />
<input type="color" onclick={() => void 0} />

<!-- Interactive element -->
<a onclick={() => void 0} />
<area onclick={() => {}} />
<button onclick={() => void 0} />
<menuitem onclick={() => {}} />
<option onclick={() => void 0} class="foo" />
<select onclick={() => void 0} class="foo" />
<textarea onclick={() => void 0} class="foo" />
<tr onclick={() => {}} />

<!-- Element attributed with an interactive role -->
<div role="button" onclick={() => {}} />
<div role="checkbox" onclick={() => {}} />
<div role="combobox" onclick={() => {}} />
<div role="gridcell" onclick={() => {}} />
<div role="link" onclick={() => {}} />
<div role="listbox" onclick={() => {}} />
<div role="menuitem" onclick={() => {}} />
<div role="menuitemcheckbox" onclick={() => {}} />
<div role="menuitemradio" onclick={() => {}} />
<div role="option" onclick={() => {}} />
<div role="radio" onclick={() => {}} />
<div role="row" onclick={() => {}} />
<div role="scrollbar" onclick={() => {}} />
<div role="searchbox" onclick={() => {}} />
<div role="slider" onclick={() => {}} />
<div role="spinbutton" onclick={() => {}} />
<div role="switch" onclick={() => {}} />
<div role="tab" onclick={() => {}} />
<div role="textbox" onclick={() => {}} />

<!-- Presentation role -->
<div role="presentation" onclick={() => {}} />

<div onclick={() => {}} aria-hidden="true" />
