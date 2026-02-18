/**
 * `correctness/noGlobalDirnameFilename` does not trigger
 * in any of the following scenarios
 */

// String concatenation
const path1 = __dirname + "/data";
const path2 = "/data" + __filename;

// Template string literals
const command: string = `cat ${__filename}`;

// Operators
const target = process.env.DEBUG && __filename;
const folder = __dirname || "./default";
const dir: string = __dirname ?? "";
const location = true ? __dirname : "unknown";

// Array initialization
const arr: string[] = [__filename];

// Computed property access
const cache: Record<string, string> = {};
const value = cache[__dirname];

// Function parameters
function logPath(path = __dirname) {
	console.log(path);
}
const arrowLog = (f = __filename) => console.log(f);

// Type Assertions
const fileAsAny = __filename as any;

// JSX
const Component = () => <div data-path={__dirname} />;

// Class properties
class FileLogger {
	currentFile = __filename;
}
new FileLogger().currentFile = __filename;
