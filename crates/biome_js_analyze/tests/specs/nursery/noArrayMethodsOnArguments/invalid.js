/* should generate diagnostics */
function staticMember() { arguments.map(() => {}); }
function stringMember() { arguments["map"](() => {}); }
function templateMember() { arguments[`map`](() => {}); }
function parenthesized() { (arguments).map(() => {}); }
function optionalMember() { arguments?.map(() => {}); }
function nestedArrow() { return () => arguments.map(() => {}); }
function twice() { arguments.map(() => {}); arguments.filter(() => {}); }
const expression = function () { arguments.at(0); };
const object = {
	method() { arguments.concat([]); },
	get getter() { return arguments.join(); },
	set setter(value) { arguments.push(value); },
};
class Foo {
	constructor() { arguments.slice(); }
	method() { arguments.some(() => {}); }
	get getter() { return arguments.every(() => {}); }
	set setter(value) { arguments.includes(value); }
}
export default function () { arguments.indexOf(0); }
function allMethods() {
	arguments.at(0);
	arguments.concat([]);
	arguments.copyWithin(0);
	arguments.entries();
	arguments.every(() => {});
	arguments.fill(0);
	arguments.filter(() => {});
	arguments.find(() => {});
	arguments.findIndex(() => {});
	arguments.findLast(() => {});
	arguments.findLastIndex(() => {});
	arguments.flat();
	arguments.flatMap(() => {});
	arguments.forEach(() => {});
	arguments.includes(0);
	arguments.indexOf(0);
	arguments.join();
	arguments.keys();
	arguments.lastIndexOf(0);
	arguments.map(() => {});
	arguments.pop();
	arguments.push(0);
	arguments.reduce(() => {});
	arguments.reduceRight(() => {});
	arguments.reverse();
	arguments.shift();
	arguments.slice();
	arguments.some(() => {});
	arguments.sort();
	arguments.splice(0);
	arguments.toReversed();
	arguments.toSorted();
	arguments.toSpliced(0);
	arguments.unshift();
	arguments.values();
	arguments.with(0, 0);
}
