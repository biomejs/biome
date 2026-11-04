/* should not generate diagnostics */
import Link from "next/link";

export function Links({ href }) {
	return (
		<>
			<Link href="/about">About, with next/link</Link>
			<A href="/about">About, with a component</A>
			<ui.a href="/about">About, with a member component</ui.a>
			<a href={href}>Dynamic href</a>
			<a href={`/blog/${href}`}>Template literal</a>
			<a href="/about" target="_blank">About, in a new tab</a>
			<a href="/about" download>About, downloaded</a>
			<a href="https://example.com/about">Other origin</a>
			<a href="//example.com/about">Protocol-relative URL</a>
			<a href="about">Relative URL</a>
			<a href="#about">Fragment</a>
			<a href="mailto:hello@example.com">Email</a>
			<a href="/unknown">Not a page</a>
			<a href="/about/team">Below a page without a dynamic segment</a>
			<a href="/blog">Dynamic segment without a value</a>
			<a href="/blog/report.pdf">Dynamic segment with a file name</a>
			<a href="/docs">Catch-all without segments</a>
			<a href="/status">Route handler</a>
			<a href="/api/users">API route</a>
			<a href="/_app">Pages router special file</a>
			<a href="/_components">Private folder</a>
			<a href="/feed/photo">Intercepting route</a>
			<a href="/marketing/pricing">Route group in the URL</a>
			<a href="/@modal/settings">Parallel route slot in the URL</a>
			<a>No href</a>
		</>
	);
}
