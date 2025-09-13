/* should not generate diagnostics */

import { useCallback } from "react";

function onClick() {}
<Foo onClick={onClick} />

const onClick = function() {}
<Foo onClick={onClick} />

const onClick = () => {}
<Foo onClick={onClick} />

<Foo onClick={this.handleClick}></Foo>;
<Foo onClick={bind()}></Foo>;

function Foo() {
    const onClick = useCallback(() => {}, []);
    return <Bar onClick={onClick} />
}
