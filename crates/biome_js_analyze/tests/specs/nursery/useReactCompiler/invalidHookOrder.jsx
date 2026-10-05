// should generate diagnostics

import { useState } from "react";

function Component(props) {
    if (props.enabled) {
        useState(0);
    }

    return <div />;
}

function EarlyReturn(props) {
    if (props.disabled) {
        return null;
    }

    useState(0);

    return <div />;
}
