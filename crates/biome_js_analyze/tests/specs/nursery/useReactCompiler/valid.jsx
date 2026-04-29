/* should not generate diagnostics */

import { useMemo, useState } from "react";

function Component(props) {
    const [count] = useState(0);
    const value = useMemo(() => props.value + count, [props.value, count]);

    return <div>{value}</div>;
}
