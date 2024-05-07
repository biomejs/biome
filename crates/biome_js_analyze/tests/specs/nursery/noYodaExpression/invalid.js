if (x <= 'foo' || 'bar' < x) {}
if ("red" == value) {}
if (true === value) {}
if (5 != value) {}
if (5n != value) {}
if (null !== value) {}
if ("red" <= value) {}
if (`red` <= value) {}
if (`red` <= `${foo}`) {}
if (`red` <= `${"red"}`) {}
if (true >= value) {}
var foo = (5 < value) ? true : false
function foo() { return (null > value); }
if (-1 < str.indexOf(substr)) {}
