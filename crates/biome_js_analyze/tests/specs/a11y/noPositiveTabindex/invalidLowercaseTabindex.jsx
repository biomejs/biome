/* should generate diagnostics */
<>
	<div tabindex={1} />
	<div tabindex={"1"} />
	<div tabindex={'5'} />
	<div tabindex="1" />
	<div tabindex={1}>foo</div>
	<div tabindex={"1"}>foo</div>
	<div tabindex={'5'}>foo</div>
	<div tabindex={+5}>foo</div>
	<div tabindex={+0o5}>foo</div>
	<div tabindex="1">foo</div>
</>
