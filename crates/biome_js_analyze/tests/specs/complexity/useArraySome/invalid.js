const items = [1, 2, 3];
const thisArg = {};
const cond = true;

items.filter(x => x > 1).length > 0;
items.filter(function (x) { return x > 1; }, thisArg).length !== 0;
items.filter(x => x > 1).length >= 1;

0 < items.filter(x => x > 1).length;
0 !== items.filter(x => x > 1).length;
0 != items.filter(x => x > 1).length;
1 <= items.filter(x => x > 1).length;

items.findIndex(x => x > 1) !== -1;
items.findLastIndex(x => x > 1) !== -1;

if (items.find(x => x > 1)) {}
if (items.findLast(x => x > 1)) {}

cond ? (items.find(x => x > 1) ? "a" : "b") : "c";

items.find(x => x > 1) !== undefined;
items.findLast(x => x > 1) !== undefined;

items.find(x => x > 1) != null;
items.findLast(x => x > 1) != null;
