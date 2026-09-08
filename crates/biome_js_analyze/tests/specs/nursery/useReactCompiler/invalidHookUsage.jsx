// should generate diagnostics

import { useEffect, useState } from "react";

function HookInNestedFunction() {
    function nested() {
        useState(0);
    }

    nested();

    return <div />;
}

function DynamicHookReference(props) {
    const hook = props.enabled ? useState : useEffect;

    hook(0);

    return <div />;
}
