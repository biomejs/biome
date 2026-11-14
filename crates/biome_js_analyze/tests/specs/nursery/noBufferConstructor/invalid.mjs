/* should generate diagnostics */
import { Buffer } from "buffer";
import { Buffer as NodeBuffer } from "node:buffer";
import * as bufferModule from "buffer";
import buffer from "node:buffer";
import { Buffer as SafeBuffer } from "safe-buffer";
import process from "node:process";

new Buffer(5);
NodeBuffer(5);
new bufferModule.Buffer(5);
buffer.Buffer(5);
new SafeBuffer(5);
new (process.getBuiltinModule("buffer").Buffer)(5);
