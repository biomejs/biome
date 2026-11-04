/* should generate diagnostics */
require("../" + name);
require(`../${name}`);
require(name);
require(name());
require(name + "foo", "bar");
require(`foo${x}`);
var foo = require(`foo${x}`);
require(String.raw`foo`);
require(...names);
(require)(name);
require?.(name);
