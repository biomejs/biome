var thing = foo ? bar : baz === qux ? quxx : foobar;

foo ? baz === qux ? quxx() : foobar() : bar();

var thing1 = foo ? (bar ? 1 : 2) : 3;

var thing2 = foo ? 1 : (bar ? 2 : 3);

var thing3 = foo ? ((bar ? 1 : 2)) : 3;

var thing4 = foo ? 1 : ((bar ? 2 : 3));

var thing5 = foo ? (baz === qux ? quxx : foobar) : bar;

const case1 = val ? (Math.random() ? 1 : 2) : 3;

const case2 = val ? 1 : Math.random() ? 2 : 3;

const case3 = (val ? (Math.random() ? 1 : 2) : 3);
