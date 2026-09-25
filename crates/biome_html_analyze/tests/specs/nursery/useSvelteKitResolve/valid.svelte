<!-- should not generate diagnostics -->
<script>
	import { resolve } from "$app/paths";
</script>

<a href={resolve("/foo")}>Click me!</a>
<a href={resolve("/blog/[slug]", { slug })}>Click me!</a>
<a href="https://svelte.dev">Click me!</a>
<a href="mailto:hello@example.com">Click me!</a>
<a href="svn+ssh://example.com">Click me!</a>
<a href={"https://svelte.dev"}>Click me!</a>
<a href="https://svelte.dev/{path}">Click me!</a>
<a href="#top">Click me!</a>
<a href={"#top"}>Click me!</a>
<a href="/foo" rel="external">Click me!</a>
<a href="/foo" rel="noopener external">Click me!</a>
<a href={"/foo"} rel={"external"}>Click me!</a>
<a href="/foo" rel={rel}>Click me!</a>
<a href="/foo" {rel}>Click me!</a>
<a href={null}>Click me!</a>
<a href={undefined}>Click me!</a>

<!-- Dynamic values aren't checked -->
<a href={url}>Click me!</a>
<a {href}>Click me!</a>
<a href="{url}/foo">Click me!</a>

<!-- No value -->
<a href>Click me!</a>
<a>Click me!</a>

<!-- Other elements -->
<link href="/foo" />
<Link href="/foo" />
