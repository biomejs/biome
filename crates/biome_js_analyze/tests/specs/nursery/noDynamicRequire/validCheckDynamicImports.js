/* should not generate diagnostics */
import("foo");
import(`foo`);
import("./foo");
import("@scope/foo");
var foo = import("foo");
var bar = import(`foo`);
import("./foo", { with: { type: "json" } });
