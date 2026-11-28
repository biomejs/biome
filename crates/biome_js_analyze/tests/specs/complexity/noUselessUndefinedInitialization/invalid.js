let test1,
	test2 = undefined;
var c = undefined,
	o = undefined;

for (let i = 0; i < 100; i++) {
	let i = undefined;
}

let x = 1, y = undefined /* comment here */, z = 40

let /* comment */d = undefined;

let e = undefined/**/ ;
let f = /**/undefined/**/ ;
let g 
  /**/= /**/undefined/**/ ;

// No fix for `var`: removing the initializer would keep the previous iteration's value.
for (let i = 0; i < 3; i++) {
	var h = undefined;
	if (i === 1) h = "set";
}

// No fix for `var`: removing the initializer would keep the previous value of the redeclared variable.
function redeclared() {
	var j = 1;
	var j = undefined;
	return j;
}
