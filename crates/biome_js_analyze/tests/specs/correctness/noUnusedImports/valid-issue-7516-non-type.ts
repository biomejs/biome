/* should not generate diagnostics */

// Issue #7516: non-type namespace import, variable shadows it
import * as unist from "unist"
const _f = () => {
    let unist: unist.Node
}
