const obj = {
	// Comment b
	b: 1,
	// Comment a
	a: 1,
	...g,
	ba: 2,
	ab: 1,
	set aab(v) {
		this._aab = v;
	},
	[getProp()]: 2,
	aba: 2,
	abc: 3,
	abb: 3,
	a10: 0,
	19: 0,
	get aaa() {
		return "";
	},
};
