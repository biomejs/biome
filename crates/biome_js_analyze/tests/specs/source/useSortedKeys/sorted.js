const obj = {
	get aab() {
		return this._aab;
	},
	set aac(v) {
		this._aac = v;
	},
	w: 1,
	x: 1,
	...g,
	get aaa() {
		return "";
	},
	u: 1,
	v: 1,
	[getProp()]: 2,
	o: 1,
	p: 1,
	q: 1,
};
