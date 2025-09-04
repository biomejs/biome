const sneakyObject2 = {
	get something() {
		return new Promise((_, reject) => reject("This is a floating promise!"));
	},
};
sneakyObject2.something;
