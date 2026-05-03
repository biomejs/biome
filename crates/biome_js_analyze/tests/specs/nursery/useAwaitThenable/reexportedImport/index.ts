/* should not generate diagnostics */

import { getValue } from "./reexport";

async function test() {
    // This should be valid - getValue returns a Promise
    await getValue();
}
