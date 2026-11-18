/* should not generate diagnostics */
const a = new Array<number>(5).fill(0).map((item) => item);
class Array {
	map(callback: () => void) {
		callback();
	}
}
new Array(5).map(() => {});
