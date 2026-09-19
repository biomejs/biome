// should generate diagnostics
import { readNumber, assert, type OptionalNumber } from "./exports";
if (readNumber()) {}
function check(value: OptionalNumber) {
    if (value) {}
    assert(value);
}
