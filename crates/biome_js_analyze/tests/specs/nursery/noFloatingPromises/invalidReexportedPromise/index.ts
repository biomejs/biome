/* should generate diagnostics */

import { getValue } from "./reexport";

function test() {
    // Invalid: Promise is not handled (no await, no .then, no void operator)
    getValue();
}
