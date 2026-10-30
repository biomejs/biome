/* should generate diagnostics */

array.forEach((element) => {
	bar(element);
});

array?.forEach((element) => bar(element));

array.forEach((element, index) => bar(element, index));

array.forEach((element, index, array) => bar(element, index, array));

array.forEach(bar);

array.forEach();

array.forEach((element) => bar(element), thisArgument);

array["forEach"]((element) => bar(element));

array.forEach?.((element) => bar(element));

(array).forEach((element) => bar(element));

(array.forEach)((element) => bar(element));

foo.bar.forEach((element) => bar(element));

(foo || bar).forEach((element) => log(element));

const result = array.forEach((element) => bar(element));

[1, 2, 3].forEach((element) => bar(element));

const typedArray = new Uint8Array();
typedArray.forEach((value) => console.log(value));

let map = new Map();
map.forEach((value) => console.log(value));

foo.React.Children.forEach(bar);

React?.Children.forEach(bar);

NotReact.Children.forEach(bar);

NotChildren.forEach(bar);

const CB = [];
CB.forEach((element) => console.log(element));

function shadowed(Map) {
	new Map().forEach((value) => console.log(value));
}
