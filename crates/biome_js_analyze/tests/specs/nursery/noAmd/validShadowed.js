/* should not generate diagnostics */
import { define } from "custom-elements";
const require = createRequire();
define(["a"], function (a) {});
require(["a"], function (a) {});
