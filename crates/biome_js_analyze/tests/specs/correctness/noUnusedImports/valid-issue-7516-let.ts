/* should not generate diagnostics */

// Issue #7516: let variable shadows import type namespace
import type * as hast from "hast"
const _f = () => {
    let hast: hast.Root
}
