/*before-not*/!/*after-not*/condition/*after-test*/
	? /*consequent-leading*/ consequent /*consequent-trailing*/
	: /*alternate-leading*/ alternate /*alternate-trailing*/;

call(
	!condition
		? consequent /*consequent-trailing*/
		: alternate
);
