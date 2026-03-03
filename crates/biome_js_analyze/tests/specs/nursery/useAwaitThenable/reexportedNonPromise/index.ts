/* should generate diagnostics */

import { getValue } from "./reexport";

async function test() {
    // This should trigger the rule - getValue returns a number, not a Promise
    await getValue();
}
