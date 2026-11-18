/* should not generate diagnostics */
const __filename = getFileName();

function resolve(__dirname) {
	return __dirname + "/foo.js";
}

var fullPath = __filename + "/foo.js";
var fullPath = `${__filename}/foo.js`;
