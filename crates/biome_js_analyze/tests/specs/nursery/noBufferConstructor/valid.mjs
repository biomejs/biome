/* should not generate diagnostics */
import { Buffer } from "some-other-buffer";
import { Blob } from "node:buffer";
import * as other from "other";

new Buffer(5);
new Blob([]);
new other.Buffer(5);

function shadowRequire(require) {
	new (require("buffer").Buffer)(5);
}
