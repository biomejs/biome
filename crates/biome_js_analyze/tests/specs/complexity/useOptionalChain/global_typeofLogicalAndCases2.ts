// chained members
typeof foo != 'undefined' && foo.bar;
typeof foo.bar != 'undefined' && foo.bar.baz;
typeof foo != 'undefined' && foo();
typeof foo.bar != 'undefined' && foo.bar();
typeof foo != 'undefined' && typeof foo.bar != 'undefined' && typeof foo.bar.baz != 'undefined' && foo.bar.baz.buzz;
typeof foo.bar != 'undefined' && typeof foo.bar.baz != 'undefined' && foo.bar.baz.buzz;

// case with a jump (i.e. a non-'undefined'ish prop)
typeof foo != 'undefined' && typeof foo.bar != 'undefined' && foo.bar.baz.buzz;
typeof foo.bar != 'undefined' && foo.bar.baz.buzz;

// case where for some reason there is a doubled up expression
typeof foo != 'undefined' && typeof foo.bar != 'undefined' && typeof foo.bar.baz != 'undefined' && typeof foo.bar.baz != 'undefined' && foo.bar.baz.buzz;
typeof foo.bar != 'undefined' && typeof foo.bar.baz != 'undefined' && typeof foo.bar.baz != 'undefined' && foo.bar.baz.buzz;

// chained members with element access
typeof foo != 'undefined' && typeof foo[bar] != 'undefined' && typeof foo[bar].baz != 'undefined' && foo[bar].baz.buzz;

// case with a jump (i.e. a non-'undefined'ish prop)
typeof foo != 'undefined' && typeof foo[bar].baz != 'undefined' && foo[bar].baz.buzz;

// chained calls
typeof foo != 'undefined' && typeof foo.bar != 'undefined' && typeof foo.bar.baz != 'undefined' && foo.bar.baz.buzz();
typeof foo != 'undefined' && typeof foo.bar != 'undefined' && typeof foo.bar.baz != 'undefined' && typeof foo.bar.baz.buzz != 'undefined' && foo.bar.baz.buzz();
typeof foo.bar != 'undefined' && typeof foo.bar.baz != 'undefined' && typeof foo.bar.baz.buzz != 'undefined' && foo.bar.baz.buzz();

// case with a jump (i.e. a non-'undefined'ish prop)
typeof foo != 'undefined' && typeof foo.bar != 'undefined' && foo.bar.baz.buzz();
typeof foo.bar != 'undefined' && foo.bar.baz.buzz();

// case with a jump (i.e. a non-'undefined'ish prop)
typeof foo != 'undefined' && typeof foo.bar != 'undefined' && typeof foo.bar.baz.buzz != 'undefined' && foo.bar.baz.buzz();

// case with a call expr inside the chain for some inefficient reason
typeof foo != 'undefined' && typeof foo.bar() != 'undefined' && typeof foo.bar().baz != 'undefined' && typeof foo.bar().baz.buzz != 'undefined' && foo.bar().baz.buzz();


// chained members (double quotes)
typeof foo != "undefined" && foo.bar;
typeof foo.bar != "undefined" && foo.bar.baz;
typeof foo != "undefined" && foo();
typeof foo.bar != "undefined" && foo.bar();
typeof foo != "undefined" && typeof foo.bar != "undefined" && typeof foo.bar.baz != "undefined" && foo.bar.baz.buzz;
typeof foo.bar != "undefined" && typeof foo.bar.baz != "undefined" && foo.bar.baz.buzz;

// chained members (backticks)
typeof foo != `undefined` && foo.bar;
typeof foo.bar != `undefined` && foo.bar.baz;
typeof foo != `undefined` && foo();
typeof foo.bar != `undefined` && foo.bar();
typeof foo != `undefined` && typeof foo.bar != `undefined` && typeof foo.bar.baz != `undefined` && foo.bar.baz.buzz;
typeof foo.bar != `undefined` && typeof foo.bar.baz != `undefined` && foo.bar.baz.buzz;
