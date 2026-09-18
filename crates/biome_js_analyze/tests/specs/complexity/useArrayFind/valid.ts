/* should not generate diagnostics */
[1, 2, 3].find(x => x > 1);
[1, 2, 3].filter(x => x > 1)[1];
const foundArray = [1, 2, 3].filter(x => x > 1)[1];

[1, 2, 3].filter(x => x > 1).concat([5, 6, 7])[0];

const obj = {
	find: () => {
		return [1, 2, 3]
	}
}
obj.find()[0];
