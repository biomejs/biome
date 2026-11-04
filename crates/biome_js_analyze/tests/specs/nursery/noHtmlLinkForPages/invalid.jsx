/* should generate diagnostics */
export function Links() {
	return (
		<>
			<a href="/">Home</a>
			<a href="/about">About</a>
			<a href="/about/">About, with a trailing slash</a>
			<a href="/about?tab=team#history">About, with a query and a fragment</a>
			<a href="/pricing">Pricing, in a route group</a>
			<a href="/blog/hello-world">Blog post</a>
			<a href="/docs/getting-started/installation">Docs, catch-all</a>
			<a href="/settings">Settings, in a parallel route slot</a>
			<a href="/contact">Contact, pages router</a>
			<a href="/shop">Shop, optional catch-all without segments</a>
			<a href="/shop/shoes/red">Shop, optional catch-all</a>
			<a href={"/about"}>About, in an expression</a>
			<a target="_self" href="/about">About, in the same tab</a>
		</>
	);
}
