<>
	<Anchor />
	<Anchor href="" onClick={() => void 0} />
	<div href="foo" />
	<a {...props} />
	<a href="foo" />
	<a href="/foo" />
	<a href="#foo" />
	<a href="javascript" />
	<a href={`#foo`} />
	<a href={"foo"} />
	<a href={foo} />
	<a href={this} />
	<a href={getFileUrl()}>Download</a>
	<a href={a ? b : c}>Download</a>
	<a href={await getLink()}>Download</a>
	<a href={a["a"]}>Download</a>
	<a href={true ?? "url"}>Download</a>
	<a href="http://www.deque.com" onClick="window.open(this.href); return false;"> Annual Reports </a>
	<a href="https://github.com" onClick={progressivelyEnhancedFunction}>This is a progressively enhanced link</a>
</>;
