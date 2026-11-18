/* should generate diagnostics */
const fullPath1 = (__dirname as string) + "/foo.js";
const fullPath2 = __dirname! + "/foo.js";
const fullPath3 = __dirname + ("/foo.js" as string);
const fullPath4 = `${__filename satisfies string}/foo.js`;
