// should generate diagnostics

import { useState } from "react";

function Component() {
    const [value, setValue] = useState(0);
    setValue(1);

    return <div>{value}</div>;
}
