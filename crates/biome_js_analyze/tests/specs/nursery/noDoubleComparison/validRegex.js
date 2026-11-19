/* should not generate diagnostics */
const regex = /foo/;
regex === b || regex < b;
b < regex || b === regex;

const flagged = /foo/g;
flagged === b || flagged < b;

const first = /foo/, second = /foo/;
first === second || first < second;

const aliased = regex;
aliased === b || aliased < b;
const aliasedTwice = aliased;
aliasedTwice === b || aliasedTwice < b;
