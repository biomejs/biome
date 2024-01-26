/* should not generate diagnostics */

for (let i = 0; i < array.length; i++) {
	console.log(i);
}

for (let i = 0; i < arr1.length; i++) {
  const x = arr1[i] === arr2[i];
}

for (let i = 0; i < arr.length; i++) {
  arr[i] = 0;
}

for (let i = 0; i < array.length; i++) {
  console.log({ i });
}

for (var c = 0; c < arr.length; c++) {
  doMath(c);
}

for (var d = 0; d < arr.length; d++) doMath(d);

for (var e = 0; e < arr.length; e++) {
  if (e > 5) {
    doMath(e);
  }
  console.log(arr[e]);
}

for (var f = 0; f <= 40; f++) {
  doMath(f);
}

for (var g = 0; g <= 40; g++) doMath(g);

for (var h = 0, len = arr.length; h < len; h++) {}

for (var i = 0, len = arr.length; i < len; i++) arr[i];


for (;;) {
  if (m > 3) break;
  console.log(m);
  m++;
}


for (; n < 9; n++) {
  console.log(n);
}

for (; o < arr.length; o++) {
  console.log(arr[o]);
}

for (; x < arr.length; x++) {}

for (let x = 0; ; x++) {}

for (let x = 0; x < arr.length; ) {}

for (let x = 0; NOTX < arr.length; x++) {}

for (let x = 0; x < arr.length; NOTX++) {}

for (let NOTX = 0; x < arr.length; x++) {}

for (let x = 0; x < arr.length; x--) {}

for (let x = 0; x <= arr.length; x++) {}

for (let x = 1; x < arr.length; x++) {}

for (let x = 0; x < arr.length(); x++) {}

for (let x = 0; x < arr.length; x += 11) {}

for (let x = arr.length; x > 1; x -= 1) {}

for (let x = 0; x < arr.length; x *= 2) {}

for (let x = 0; x < arr.length; x = x + 11) {}

for (let x = 0; x < arr.length; x++) {
  x++;
}

for (let x = 0; true; x++) {}

for (var q in obj) {
  if (obj.hasOwnProperty(q)) {
    console.log(q);
  }
}

for (var r of arr) {
  console.log(r);
}

for (let x = 0; x < arr.length; x++) {
  let y = arr[x + 1];
}

for (let i = 0; i < arr.length; i++) {
  delete arr[i];
}

for (let i = 0; i < arr.length; i++) {
  [arr[i]] = [1];
}

for (let i = 0; i < arr.length; i++) {
  [...arr[i]] = [1];
}

for (let i = 0; i < arr1?.length; i++) {
  const x = arr1[i] === arr2[i];
}

for (let i = 0; i < arr?.length; i++) {
  arr[i] = 0;
}

for (var c = 0; c < arr?.length; c++) {
  doMath(c);
}

for (var d = 0; d < arr?.length; d++) doMath(d);

for (var c = 0; c < arr.length; c++) {
  doMath?.(c);
}

for (var d = 0; d < arr.length; d++) doMath?.(d);

for (let i = 0; i < test.length; ++i) {
  this[i];
}

for (let i = 0; i < this.length; ++i) {
  yield this[i];
}
