// should generate diagnostics

import { useEffect, useState } from "react";

function Component(props) {
    const hook = props.enabled ? useState : useEffect;

    hook(0);

    return <div />;
}
