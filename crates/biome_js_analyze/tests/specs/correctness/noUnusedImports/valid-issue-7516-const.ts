/* should not generate diagnostics */

// Issue #7516: const variable shadows import type namespace
import type * as mdast from "mdast"
const _f = () => {
    const mdast: mdast.Node = {} as any
}
