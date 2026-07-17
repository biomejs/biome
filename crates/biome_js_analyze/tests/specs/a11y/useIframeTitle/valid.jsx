/* should not generate diagnostics */
<>
	<iframe title="This is a unique title" />
	<iframe title={uniqueTitle} />
	{/* this case might contain `title` attribute */}
	<iframe {...{ title: "title" }} />
	<iframe {...props} />
</>;
