/* should not generate diagnostics */

import { getValue } from "./reexport";

async function test() {
    // Valid: Promise is properly handled with await
    await getValue();
}
