// should generate diagnostics

import { useMemo } from "react";

function Component(props) {
    const value = useMemo(async () => props.value, [props.value]);

    return <div>{value}</div>;
}
