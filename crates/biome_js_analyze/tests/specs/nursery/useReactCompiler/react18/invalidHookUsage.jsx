// should not generate diagnostics

import { useState } from "react";

function HookInNestedFunction() {
    function nested() {
        useState(0);
    }

    nested();

    return <div />;
}
