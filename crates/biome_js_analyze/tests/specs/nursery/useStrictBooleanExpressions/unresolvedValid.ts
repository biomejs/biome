// should not generate diagnostics
import { missing } from "./missing";
if (missing) {}
if (notDeclared) {}
function unresolved(value: MissingType | undefined) {
    if (value) {}
}
