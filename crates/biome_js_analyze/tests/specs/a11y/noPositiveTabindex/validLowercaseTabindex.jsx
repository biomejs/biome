/* should not generate diagnostics */
<>
	<div />
	<div {...props} />
	<div tabindex={func} />
	<div tabindex={func()} />
	<div tabindex={null} />
	<div tabindex={undefined} />
	<div tabindex={"abc"} />
	<div tabindex={0} />
	<div tabindex={+0} />
	<div tabindex={-0} />
	<div tabindex={-1} />
	<div tabindex={"0"} />
	<div tabindex={'-5'} />
	<div tabindex={.5} />
	<div tabindex={-5.5} />
	<div tabindex={`-1`} />
	<div tabindex={`1`} />
	<div tabindex="-1" />
	<div tabindex="abc" />
	<div tabindex={typeof e} />
	<div>foo</div>
	<div tabindex={-1}>foo</div>
	<div tabindex={"-1"}>foo</div>
	<div tabindex={'-5'}>foo</div>
	<div tabindex="-1">foo</div>
	<div tabindex={`-1`}>foo</div>
	<div tabindex={`1`}>foo</div>
	<div tabindex={`abc`}>foo</div>
	<div tabindex={null}>foo</div>
	<div tabindex={undefined}>foo</div>
	<div tabindex={func()}>foo</div>
	<div tabindex={func}>foo</div>
	<div tabindex={typeof e}>foo</div>
	<div {...props}>foo</div>
</>
