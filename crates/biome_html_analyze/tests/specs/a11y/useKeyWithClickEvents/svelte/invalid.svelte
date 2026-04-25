<!-- should generate diagnostics -->

<!-- div with onclick but no keyboard handler -->
<div onclick="handleClick()"></div>
<div onclick={handleClick}></div>
<div onclick={() => {}}></div>

<!-- span with onclick but no keyboard handler -->
<span onclick="handleClick()">text</span>

<!-- <a> without href is not interactive -->
<a onclick="handleClick()">bare link</a>
