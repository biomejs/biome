/* should generate diagnostics */
foo[i++] |= 0;
getObject().value |= 0;
foo[bar()] |= 0;
foo[delete a.b] |= 0;
foo[a = 1] |= 0;
[...foo][0] |= 0;
foo /* comment */ |= 0;
foo |= /* comment */ 0;
foo[tag`key`] |= 0;
foo[`${bar()}`] |= 0;
