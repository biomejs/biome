import * as ReactTypes from "react";

function Component() {
    const onClick = (event: ReactTypes.MouseEvent) => { };

    return <div onClick={onClick}></div>;
}
