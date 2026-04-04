/* should not generate diagnostics */

import { useCallback } from "react";

function onClick() {}

function Foo() {
	return <Bar onClick={onClick} />;
}

const onClick = function () {};

function Foo() {
	return <Bar onClick={onClick} />;
}

const onClick = () => {};

function Foo() {
	<Bar onClick={onClick} />;
}

<>
	<Foo onClick={this.handleClick}></Foo>
	<Foo onClick={bind()}></Foo>
</>

function Foo() {
	const onClick = useCallback(() => {}, []);
	return <Bar onClick={onClick} />;
}
