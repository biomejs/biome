/* should not generate diagnostics */

import { useEffect } from "react";

const KEY = "key";

// Issue #9744: Computed property destructuring should not cause false positives
function Component(props) {
    const { [KEY]: value } = props;
    useEffect(() => {
        console.log(value);
    }, [value]);

    return null;
}
