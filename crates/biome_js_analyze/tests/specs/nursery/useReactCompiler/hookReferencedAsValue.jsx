// should generate diagnostics

import { useState } from "react";

function Component() {
    const hook = useState;

    return <button onClick={hook}>Click</button>;
}
