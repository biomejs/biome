/* should generate diagnostics */
var path = require("path");
const nodePath = require("node:path");
const { sep } = require("path");
const { sep: separator } = require("node:path");

var fullPath = `${__dirname}${path.sep}foo.js`;
var fullPath = `${__filename}${path.sep}foo.js`;
var fullPath = __dirname + path.sep + `foo.js`;
var fullPath = __dirname + nodePath.sep + "foo.js";
var fullPath = __dirname + require("path").sep + "foo.js";
var fullPath = __dirname + sep + "foo.js";
var fullPath = __dirname + separator + "foo.js";
