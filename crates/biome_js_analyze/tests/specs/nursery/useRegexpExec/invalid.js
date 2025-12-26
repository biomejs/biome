'something'.match(/thing/);

'some things are just things'.match(/thing/);

'something'.match(new RegExp(/thing/));

const text = 'something';
const search = /thing/;
text.match(search);

const text1 = 'something';
const search1 = new RegExp(/thing/);
text1.match(search1);
