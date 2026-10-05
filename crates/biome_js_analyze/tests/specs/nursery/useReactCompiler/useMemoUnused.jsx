// should generate diagnostics

import { useMemo } from "react";

function Component(props) {
    useMemo(() => props.value, [props.value]);

    return <div>{props.value}</div>;
}
