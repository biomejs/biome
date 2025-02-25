'foo' + 'bar';

// tagged template
number`5` + 1 + number`4`;

`a` + `b`;

`a` + "b";

("a" * 2) + 5;

//a
/*b*/ foo /*c*/ + /*d*/ 'baz' /*e*/ + /*f*/ 1 //g
+ //h
bar //i

// https://github.com/biomejs/biome/issues/4947
`${VALUE} Proident ad laborum dolor adipisicing. Consequat consequat eu deserunt in ea eiusmod.`
+ 'Veniam qui aliqua laborum tempor occaecat sunt pariatur labore esse deserunt. Aliqua fugiat';

`${VALUE} Proident ad laborum dolor adipisicing. Consequat consequat eu deserunt in ea eiusmod.` +
'Veniam qui aliqua laborum tempor occaecat sunt pariatur labore esse deserunt. Aliqua fugiat';
