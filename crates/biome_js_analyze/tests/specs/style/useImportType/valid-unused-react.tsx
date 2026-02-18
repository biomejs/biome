/* should not generate diagnostics */
import React from "react";

// https://github.com/biomejs/biome/issues/9020
import React from "./not-actually-react";

function Component() {
    const onClick = (event: React.MouseEvent) => { };

    return <div onClick={onClick}></div>;
}
