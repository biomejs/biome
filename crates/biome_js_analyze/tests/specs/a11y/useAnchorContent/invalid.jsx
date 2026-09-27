/* should generate diagnostics */
<>
	<a />
	<a></a>
	<a>   </a>
	<a>{null}</a>
	<a>{undefined}</a>
	<a aria-label="" />
	<a aria-label="   " />
	<a aria-label={""} />
	<a aria-label={null} />
	<a aria-hidden>content</a>
	<a><span aria-hidden="true">content</span></a>
	<a><span aria-hidden={true}>content</span></a>
	<a><span aria-hidden={"true"}>content</span></a>
	<a><span aria-hidden={`true`}>content</span></a>
	<a><span aria-hidden={`${true}`}>content</span></a>
	<my-button render={<a />} />
</>
