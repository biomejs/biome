/* should not generate diagnostics */

import { getArray } from "./reexport";

function test() {
    // Valid: Array sorted with compare function
    getArray().sort((a, b) => a.localeCompare(b));
}
