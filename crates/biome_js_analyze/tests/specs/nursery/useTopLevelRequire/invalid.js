/* should generate diagnostics */
if (process.env.NODE_ENV === 'DEVELOPMENT') {
	require('debug');
}
var x; if (y) { x = require('debug'); }
var x2; if (y) { x2 = require('debug').baz; }
function f() { require('y') }
try { require('x'); } catch (e) { console.log(e); }
var getModule = x => require(x);
var weird = (x => require(x))('weird')
switch(x) { case '1': require('1'); break; }
{ require('block'); }
class A { static { require('static-block'); } }
var obj = { key: require('y') };
var arr = [require('y')];
var both = a || require('y');
var seq = (a, require('y'));
var inst = new Foo(require('y'));
var tagged = require('y')`template`;
var { withDefault = require('y') } = obj;
for (const item of require('y')) {}
label: require('y');
export const exported = require('y');
