// should generate diagnostics

import { useRef } from "react";

function Component() {
    const ref = useRef(null);

    return <div>{ref.current}</div>;
}
