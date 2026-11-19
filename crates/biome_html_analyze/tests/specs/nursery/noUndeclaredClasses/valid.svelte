<!-- should not generate diagnostics -->
<style>
	.card { border: 1px solid; }
	.header { font-size: 2rem; }
	.footer { font-size: 0.8rem; }

	@media (max-width: 768px) {
		.mobile { display: block; }
	}

	.parent .child { color: red; }
	.button:hover { opacity: 0.8; }
	.active.highlighted { background: yellow; }
	.alpha + .beta { margin-top: 8px; }
	.any-class { display: block; }
</style>

<div class="card">Single defined class</div>
<div class="header footer">Multiple defined classes</div>
<div class="mobile">Responsive class</div>

<div class="parent">
	<span class="child">Nested</span>
</div>

<div class="button">Button with hover state</div>
<div class="active highlighted">Combined selector classes</div>

<div class="alpha">
	<div class="beta">Sibling</div>
</div>

<div CLASS="card">Uppercase CLASS</div>
<div Class="footer">Mixed case Class</div>

<MyComponent class="undefined-class">Uppercase component</MyComponent>
<my-component class="another-undefined">Hyphenated component</my-component>

<div class="">No classes</div>
<div class="   ">Only whitespace</div>
<div class="any-class">Unstyled section</div>
