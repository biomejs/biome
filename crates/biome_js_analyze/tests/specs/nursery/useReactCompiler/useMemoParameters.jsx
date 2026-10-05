// should generate diagnostics

import { useMemo } from "react";

function Component(props) {
    const value = useMemo((item) => item.value, [props.item]);

    return <div>{value}</div>;
}
