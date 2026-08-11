/* should generate diagnostics */
<>
	<button />
	<button></button>
	<menuitem />
	<button>   </button>

	{/* An empty, whitespace-only, `null` or `undefined` labeling attribute is not a real label. */}
	<button aria-label="" />
	<button aria-label="   " />
	<button aria-label={``} />
	<button aria-label={null} />
	<button aria-label={undefined} />
	<button aria-labelledby="" />
	<button title="" />

	{/* A falsy `aria-hidden` leaves the control exposed, so a label is still required. */}
	<button aria-hidden="false" />
	<button aria-hidden={false} />
	<button aria-hidden="" />

	{/* A hidden child supplies no accessible label, however deep the text is. */}
	<button><span aria-hidden="true">Delete</span></button>
	<button><span aria-hidden="true"><span>Delete</span></span></button>

	{/* An empty child element renders nothing announceable, at any depth. */}
	<button><span /></button>
	<button><span></span></button>
	<button><span><em></em></span></button>

	{/* A decorative or unlabeled image supplies no accessible name. */}
	<button><img /></button>
	<button><img alt="" /></button>

	{/* Void elements and hidden inputs render nothing. */}
	<button><br /></button>
	<button><input type="hidden" /></button>

	{/* A falsy expression child, a JSX comment, an empty fragment, and a falsy
	    `children` prop all render nothing. */}
	<button>{false}</button>
	<button>{/* comment */}</button>
	<button><></></button>
	<button children="" />
</>
