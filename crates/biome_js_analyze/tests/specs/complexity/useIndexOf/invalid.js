const list = ['foo', 'bar', 'baz'];
list.findIndex(bar => bar === 'foo');
list.findIndex((x) => x === 'foo');
list.findIndex(x => 'foo' === x);
list.findIndex(x => {
	return x === 'foo';
});
list.findIndex((x, y) => x === 'foo');
list.findIndex(x => x === undefined);

list.findLastIndex(bar => bar === 'foo');
list.findLastIndex((x) => x === 'foo');
list.findLastIndex(x => 'foo' === x);
list.findLastIndex(x => {
	return x === 'foo';
});
list.findLastIndex((x, y) => x === 'foo');
list.findLastIndex(x => x === undefined);
