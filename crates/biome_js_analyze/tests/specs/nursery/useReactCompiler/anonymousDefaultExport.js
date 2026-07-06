/* should not generate diagnostics */

// Known limitation of the default `infer` compilation mode: an anonymous
// default export has no name to match React conventions against, so it is
// not analyzed. Name the component, or use `compilationMode: "all"`.

import { useState } from "react";

export default (props) => {
    if (props.enabled) {
        useState(0);
    }
    return null;
};
