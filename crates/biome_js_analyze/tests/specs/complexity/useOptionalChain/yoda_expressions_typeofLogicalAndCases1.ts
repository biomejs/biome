// chained members
'undefined' !== typeof foo && foo.bar;
'undefined' !== typeof foo.bar && foo.bar.baz;
'undefined' !== typeof foo && foo();
'undefined' !== typeof foo.bar && foo.bar();
'undefined' !== typeof foo && 'undefined' !== typeof foo.bar && 'undefined' !== typeof foo.bar.baz && foo.bar.baz.buzz;
'undefined' !== typeof foo.bar && 'undefined' !== typeof foo.bar.baz && foo.bar.baz.buzz;

// case with a jump (i.e. a non-'undefined'ish prop)
'undefined' !== typeof foo && 'undefined' !== typeof foo.bar && foo.bar.baz.buzz;
'undefined' !== typeof foo.bar && foo.bar.baz.buzz;

// case where for some reason there is a doubled up expression
'undefined' !== typeof foo && 'undefined' !== typeof foo.bar && 'undefined' !== typeof foo.bar.baz && 'undefined' !== typeof foo.bar.baz && foo.bar.baz.buzz;
'undefined' !== typeof foo.bar && 'undefined' !== typeof foo.bar.baz && 'undefined' !== typeof foo.bar.baz && foo.bar.baz.buzz;

// chained members with element access
'undefined' !== typeof foo && 'undefined' !== typeof foo[bar] && 'undefined' !== typeof foo[bar].baz && foo[bar].baz.buzz;

// case with a jump (i.e. a non-'undefined'ish prop)
'undefined' !== typeof foo && 'undefined' !== typeof foo[bar].baz && foo[bar].baz.buzz;

// chained calls
'undefined' !== typeof foo && 'undefined' !== typeof foo.bar && 'undefined' !== typeof foo.bar.baz && foo.bar.baz.buzz();
'undefined' !== typeof foo && 'undefined' !== typeof foo.bar && 'undefined' !== typeof foo.bar.baz && 'undefined' !== typeof foo.bar.baz.buzz && foo.bar.baz.buzz();
'undefined' !== typeof foo.bar && 'undefined' !== typeof foo.bar.baz && 'undefined' !== typeof foo.bar.baz.buzz && foo.bar.baz.buzz();

// case with a jump (i.e. a non-'undefined'ish prop)
'undefined' !== typeof foo && 'undefined' !== typeof foo.bar && foo.bar.baz.buzz();
'undefined' !== typeof foo.bar && foo.bar.baz.buzz();

// case with a jump (i.e. a non-'undefined'ish prop)
'undefined' !== typeof foo && 'undefined' !== typeof foo.bar && 'undefined' !== typeof foo.bar.baz.buzz && foo.bar.baz.buzz();

// case with a call expr inside the chain for some inefficient reason
'undefined' !== typeof foo && 'undefined' !== typeof foo.bar() && 'undefined' !== typeof foo.bar().baz && 'undefined' !== typeof foo.bar().baz.buzz && foo.bar().baz.buzz();


// chained members (double quotes)
"undefined" !== typeof foo && foo.bar;
"undefined" !== typeof foo.bar && foo.bar.baz;
"undefined" !== typeof foo && foo();
"undefined" !== typeof foo.bar && foo.bar();
"undefined" !== typeof foo && "undefined" !== typeof foo.bar && "undefined" !== typeof foo.bar.baz && foo.bar.baz.buzz;
"undefined" !== typeof foo.bar && "undefined" !== typeof foo.bar.baz && foo.bar.baz.buzz;

// chained members (backticks)
`undefined` !== typeof foo && foo.bar;
`undefined` !== typeof foo.bar && foo.bar.baz;
`undefined` !== typeof foo && foo();
`undefined` !== typeof foo.bar && foo.bar();
`undefined` !== typeof foo && `undefined` !== typeof foo.bar && `undefined` !== typeof foo.bar.baz && foo.bar.baz.buzz;
`undefined` !== typeof foo.bar && `undefined` !== typeof foo.bar.baz && foo.bar.baz.buzz;
