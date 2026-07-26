<!-- Collapses when the expression is the name, quoted or not. -->
<input value={value} />
<input value="{value}" />
<input bind:value={value} bind:other="{other}" />
<input class:active={active} style:color="{color}" />
<Widget let:foo={foo} let:bar="{bar}" />

<!-- Left alone: the expression is not the name. -->
<input value={other} />
<input bind:value={other} />
<input class:active={isActive} />
<Widget let:foo={bar} />

<!-- Left alone: the expression is not a lone identifier. -->
<input value={a + b} />
<input class:active={!isValid} />

<!-- The quotes go, but the value stays. -->
<div style:background="{color}"></div>
<!-- The quotes stay: the value is more than one expression. -->
<div style:background='prefix{color}suffix'></div>
<div class="a {b} c"></div>
