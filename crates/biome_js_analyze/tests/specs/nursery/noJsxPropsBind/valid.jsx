/* should not generate diagnostics */

import { useCallback } from "react";

function Foo() {
	function onClick() { }
	return <Bar onClick={onClick} />
}

function Foo() {
	const onClick = function () { }
	return <Bar onClick={onClick} />
}

function Foo() {
	const onClick = () => { }
	<Bar onClick={onClick} />
}

<>
	<Foo onClick={this.handleClick}></Foo>;
	<Foo onClick={bind()}></Foo>;
</>

function Foo() {
	const onClick = useCallback(() => { }, []);
	return <Bar onClick={onClick} />
}
