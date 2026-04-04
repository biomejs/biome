/* should not generate diagnostics */

import { useEffect } from "react";

// Issue #8967: Rest pattern destructuring should not cause false positives
function Component(props) {
    const { data, ...restProps } = props;
    const { prop1 } = restProps;

    useEffect(() => {
        console.log(prop1);
    }, [prop1]);

    useEffect(() => {
        console.log(restProps.prop1);
    }, [restProps.prop1]);

    return null;
}
