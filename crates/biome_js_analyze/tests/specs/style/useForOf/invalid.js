

for (let i = 0; i < array.length; i++) {
	console.log(array[i])
}

for (var a = 0; a < obj.arr.length; a++) {
  console.log(obj.arr[a]);
}

for (var b = 0; b < arr.length; b++) console.log(arr[b]);

for (let a = 0; a < arr.length; a++) {
  console.log(arr[a]);
}

for (var b = 0; b < arr.length; b++) console?.log(arr[b]);


for (let a = 0; a < arr.length; a++) {
  console?.log(arr[a]);
}

for (let a = 0; a < arr.length; ++a) {
  arr[a].whatever();
}

for (let x = 0; x < arr.length; x++) {
	console.log(arr[x])
}

for (let x = 0; x < arr.length; x += 1) {
	console.log(arr[x])
}

for (let x = 0; x < arr.length; x = x + 1) {}

for (let shadow = 0; shadow < arr.length; shadow++) {
  for (let shadow = 0; shadow < arr.length; shadow++) {}
}

for (let i = 0; i < arr.length; i++) {
  obj[arr[i]] = 1;
}

for (let i = 0; i < arr.length; i++) {
  delete obj[arr[i]];
}

for (let i = 0; i < arr.length; i++) {
  [obj[arr[i]]] = [1];
}

for (let i = 0; i < arr.length; i++) {
  [...obj[arr[i]]] = [1];
}

for (let i = 0; i < arr.length; i++) {
  ({ foo: obj[arr[i]] } = { foo: 1 });
}

for (let i = 0; i < this.item.length; ++i) {
  this.item[i];
}

for (let i = 0; i < this.array.length; ++i) {
  yield this.array[i];
}
