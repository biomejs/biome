<!-- should generate diagnostics -->
<script>
	let a = 'hello!';
</script>

{{ a }}
{[a]}
{() => a}
{function () {
	return a;
}}
{class A {}}
{({ a })}
<p>text {{ a: true }}</p>
{#if a}{{ a }}{/if}
<input class="{{ a }} a" />
<input class="{[a]} a" />
<input class="{() => a} a" />
<input class="{class B {}} a" />
<input class="a {{ a }}" />
