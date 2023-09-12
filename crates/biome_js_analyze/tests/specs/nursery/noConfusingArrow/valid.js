/* should not generate diagnostics */

var x = a => (1 ? 2 : 3);

var x = (a) => 1 ? 2 : 3;

var x = (a) => (1 ? 2 : 3);

var x = (a) => { return 1 ? 2 : 3; };

var x = a => { return 1 ? 2 : 3; };
