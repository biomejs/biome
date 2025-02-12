foo + 'baz';

1 * 2 + 'foo';

1 + 2 + 3 + "px" + 4 + 5;

a + b + c + 'px' + d + e;

1 + 'foo' + 2 + 'bar' + 'baz' + 3;

(1 + 'foo') * 2;

1 * (2 + 'foo') + 'bar';

'foo' + 1;

'foo' + `bar${`baz${'bat' + 'bam'}`}` + 'boo';

'foo' + 1 + 2;

1 + '2' - 3;

foo() + ' bar';

foo() + '\n';

1 * /**leading*/'foo'    /**trailing */                   + 'bar';

`foo` + 1;

`foo${1}` + 2;

// strings including `${`

'${foo.' + bar + '.baz}';

'foo: ${bar.' + baz + '.bat}';

'foo: `bar.' + baz + '.bat}';

'${foo}: `bar.' + baz + '.bat}';

'foo: ${bar.' + baz + '.bat}';

'foo: `bar.' + baz + '.bat}';

'foo: \\${bar.' + baz + '.bat}';

'foo: \\${bar.' + baz + '.bat}';

// parentheses

const x = a + ("b") + c;

("a") + (b) + ("c");

//a
/*b*/ foo /*c*/ + /*d*/ 'baz' /*e*/ + /*f*/ 1 //g

// parentheses and type coercion

"a" + (1 + 2); // `a${1 + 2}`

(1 + 2) + "a"; // `${1 + 2}a`

1 + 2 + "a"; // `${1 + 2}a`

(1 + 2) + ("a"); // `${1 + 2}a`

"a" + 1 + 2; // `a${1}${2}`

1 + (2 + "a"); // `${1}${2}a`

(1 + 2) + (3 + 4) + "a"; // `${(1 + 2) + (3 + 4)}a`

(1 + 2) + ((3 + 4) + "a"); // `${1 + 2}${3 + 4}a`

"a" + (1 + 2 + "b"); // `a${1 + 2}b`

(1 + 2 + "a") + "b"; // `${1 + 2}ab`

("a" + 1 + 2) + "b"; // `a${1}${2}b`

("a" + 1) + (2 + "b"); // `a${1}${2}b`

(1 + 2) + (3 + "a") + 4 + (5 + ("b" + 6)); // `${1 + 2}${3}a${4}${5}b${6}`

"a" + 1 + (2 + (3 + 4)); // `a${1}${2 + (3 + 4)}`

1 + (2 + (3 + 4)) + "a"; // `${1·+·(2·+·(3·+·4))}a`

1 - 2 + "a"; // `${1 - 2}a`

"a" + (1 - 2); // `a${1 - 2}`

(foo && bar) + "baz"; // `${foo && bar}baz`

"foo" + (bar && baz); // `foo${bar && baz}`

(1 + foo && bar) + "baz"; // `${1 + foo && bar}baz`

"foo" + (bar && baz + 1); // `foo${bar && baz + 1}`
