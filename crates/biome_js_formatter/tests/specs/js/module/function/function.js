function foo() {
}
async function foo(a) {
	await x
}
x = function() {
}
x = async function* foo(a) {
}
function Foo() {
  if (!new.target) { }
}
function* Foo() {
  yield
  yield x
  yield* x
  yield aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
  yield* aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
}
function foo() {
	let [ref, setRef] = useState();

	useEffect(() => {
		setRef()
	});

	return ref;
}

function directives() {
  "use strict";
}
