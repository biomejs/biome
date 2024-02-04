export function foo() {
	let x: number;
	while ((x = Math.random() > 0.1)) {
		console.log(x);
	}
}
