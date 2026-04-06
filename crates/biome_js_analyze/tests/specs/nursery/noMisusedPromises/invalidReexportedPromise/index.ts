/* should generate diagnostics */

import { getValue } from "./reexport";

function test() {
    // Invalid: Promise used in boolean context (if condition)
    if (getValue()) {
        console.log("test");
    }
}
