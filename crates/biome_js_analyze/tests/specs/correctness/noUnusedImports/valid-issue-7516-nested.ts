/* should not generate diagnostics */

// Issue #7516: nested function scope with variable shadowing import type namespace
import type * as vfile from "vfile"
const _f = () => {
    const inner = () => {
        let vfile: vfile.Value
    }
}
