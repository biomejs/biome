/* should not generate diagnostics */

// Issue #7516: var variable shadows import type namespace
import type * as estree from "estree"
const _f = () => {
    var estree: estree.Program
}
