/* should not generate diagnostics */
var x = require('y');
if (x) { x.require('y'); }
var a;
a = require('y');
var b = 1, c = require('y');
var d = require('y'), e = require('y'), f = require('z');
var g = require('y').foo;
require('y').foo();
require('y');
function h(){}
h();
if (h > c) {
	doSomething()
}
var i = require('y').foo;
var logger = require(DEBUG ? 'dev-logger' : 'logger');
var logger2 = DEBUG ? require('dev-logger') : require('logger');
function localScopedRequire(require) { require('y'); }
var someFunc = require('./someFunc'); someFunc(function(require) { return('bananas'); });
var j = require('y')?.foo;
var k = require?.('y');
var l = (require)('y');
var m = (require('y'));
var n = require('y')[name];
var { o, p } = require('y');
const q = require('y'), r = require('z');
let s = require('y');
module.exports = require('y');
exports.t = require('y');
wrap(require('y'));
require('y')('z');
function nested() {
	const require = createRequire(import.meta.url);
	return require('y');
}
require('events').defaultMaxListeners = 20;
require('util').inspect.defaultOptions.depth = null;
require('y')[key] = value;
obj[require('y')] = value;
