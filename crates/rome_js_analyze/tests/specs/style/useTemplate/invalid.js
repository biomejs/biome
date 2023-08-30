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
