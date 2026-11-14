/* should generate diagnostics */
const buffer = require("buffer");
new buffer.Buffer(5);
buffer.Buffer(5);

const { Buffer: NodeBuffer } = require("node:buffer");
new NodeBuffer(5);

const SafeBuffer = require("safe-buffer").Buffer;
new SafeBuffer(5);

const AliasedBuffer = Buffer;
new AliasedBuffer(5);

new (require("buffer").Buffer)(5);
require("buffer").Buffer(5);
new (process.getBuiltinModule("node:buffer").Buffer)(5);

function shorthand() {
	const { Buffer } = require("buffer");
	new Buffer(5);
}

const { Buffer: GlobalBuffer } = global;
new GlobalBuffer(5);

const builtinBuffer = process.getBuiltinModule("buffer");
new builtinBuffer.Buffer(5);
