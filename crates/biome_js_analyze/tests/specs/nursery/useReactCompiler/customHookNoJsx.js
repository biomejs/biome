/* should generate diagnostics */

// Custom hooks are analyzed even in plain modules without JSX.

import { useState } from "react";

export function useFoo(enabled) {
    if (enabled) {
        useState(0);
    }
    return null;
}
