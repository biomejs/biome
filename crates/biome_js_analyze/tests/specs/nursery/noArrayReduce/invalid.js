/* should generate diagnostics */
array.reduce((str, item) => str += item, "");
array.reduceRight((str, item) => str += item, "");
array?.reduce((str, item) => str += item, "");
array.reduce((obj, item) => {
	obj[item] = null;
	return obj;
}, {});
array.reduce((obj, item) => ({ [item]: null }), {});
const hyphenate = (str, char) => `${str}-${char}`;
["a", "b", "c"].reduce(hyphenate);
array.reduce(reducer);
array.reduce(reducer, initialValue);
array.reduceRight(reducer, initialValue);
array.reduce(object.callback, initialValue);
array.reduce(callback.bind(undefined), initialValue);
array.reduce(async (total, item) => transform(total, item), initialValue);
getArray().reduce(callback, initialValue);
(array.reduce)(callback);
new Int32Array().reduce(callback);
// The function body has more than a `return` statement.
array.reduce((total, item) => {
	return total + doComplicatedThings(item);
	function doComplicatedThings(item) {
		return item + 1;
	}
}, 0);
// A `return` statement without a value is not a simple operation.
array.reduce((a, b) => { return; });
array.reduce(function (a, b) { return; }, initialValue);
// A logical operation is not a simple operation.
array.reduce((a, b) => a || b);
[].reduce.call(array, (s, i) => s + i);
[].reduce.call(array, sum);
[].reduce.call(sum);
[].reduceRight.call(array, sum);
Array.prototype.reduce.call(array, (s, i) => s + i);
Array.prototype.reduce.call(array, sum);
Array.prototype.reduceRight.call(array, sum);
[].reduce.apply(array, [(s, i) => s + i]);
[].reduce.apply(array, [sum]);
Array.prototype.reduce.apply(array, [(s, i) => s + i]);
Array.prototype.reduceRight.apply(array, [sum]);
array.reduce((a, b) => a && b);
array.reduce((a, b) => a ?? b);
