class Greeter {
	constructor(param: Array<any>) {}
}

class Greeter {
	message: any;
}

class Greeter {
	message: Array<any>;
}

class Greeter {
	message: any[];
}

class Greeter {
	message: Array<Array<any>>;
}

class Greeter {
	message: Array<any[]>;
}

class Foo<t = any> extends Bar<any> {}

abstract class Foo<t = any> extends Bar<any> {}

abstract class Foo<t = any> implements Bar<any>, Baz<any> {}

new Foo<any>()

Foo<any>()
