const sneakyObject1 = {
	doSomething() {
		return Promise.resolve("This is a floating promise!");
	},
};
sneakyObject1.doSomething();
