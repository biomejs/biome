/* should not generate diagnostics */
const array = [];

for (const value of array) {
	console.log(value);
}

for (let i = 0; i < array.length; i += 1) {
	console.log(i, array[i]);
}

array.forEach((value, i) => {
	console.log(i, value);
});

for (const [i, value] of array.entries()) {
	console.log(i, value);
}
