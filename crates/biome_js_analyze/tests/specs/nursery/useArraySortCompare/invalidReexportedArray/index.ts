/* should generate diagnostics */

import { getArray } from "./reexport";

function test() {
    // Invalid: Number array sorted without compare function
    getArray().sort();
}
