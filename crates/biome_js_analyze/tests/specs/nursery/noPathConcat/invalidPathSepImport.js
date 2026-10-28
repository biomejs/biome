/* should generate diagnostics */
import path from "node:path";
import * as pathNamespace from "path";
import pathDefault, { sep, sep as separator } from "node:path";

var fullPath = __dirname + path.sep + "foo.js";
var fullPath = __dirname + path["sep"] + "foo.js";
var fullPath = `${__dirname}${path.sep}foo.js`;
var fullPath = __filename + pathNamespace.sep + "foo.js";
var fullPath = __dirname + pathDefault.sep + "foo.js";
var fullPath = __dirname + sep + "foo.js";
var fullPath = `${import.meta.dirname}${separator}foo.js`;
var fullUrl = import.meta.url + sep + "foo.js";
