// should generate diagnostics

import { useMemo } from "react";

function MissingReturn() {
    const value = useMemo(() => {}, []);

    return <div>{value}</div>;
}

function UnusedMemo(props) {
    useMemo(() => props.value, [props.value]);

    return <div>{props.value}</div>;
}

function DynamicDependencies(props) {
    const value = useMemo(
        () => props.text.toUpperCase(),
        props.enabled ? [props.text] : null,
    );

    return <div>{value}</div>;
}
