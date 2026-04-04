// should not generate diagnostics
const items = [1, 2, 3];
const found = items.find(x => x > 1);
const foundLast = items.findLast(x => x > 1);

items.some(x => x > 1);

items.filter(x => x > 1).length === 0;
items.filter(x => x > 1).length > 1;
0 === items.filter(x => x > 1).length;
1 > items.filter(x => x > 1).length;

items.findIndex(x => x > 1) === -1;
items.findLastIndex(x => x > 1) === -1;

items.find(x => x > 1) === undefined;
items.findLast(x => x > 1) === undefined;

items.find(x => x > 1) == null;
items.findLast(x => x > 1) == null;

found;
foundLast;
