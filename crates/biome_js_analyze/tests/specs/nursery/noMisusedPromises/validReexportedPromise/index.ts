/* should not generate diagnostics */

import { getValue } from "./reexport";

async function test() {
    // Valid: Promise returned from async function
    return getValue();
}
