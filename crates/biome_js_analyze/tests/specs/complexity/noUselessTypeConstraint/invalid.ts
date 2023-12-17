interface FooAny1<T extends any> {
	field: T;
}

interface FooAny2<T extends unknown> {
	field: T;
}

class BazAny<T extends any> {
  quxAny<U extends any>() {}
}

const QuuxAny = <T extends any>() => {};

function QuuzAny<T extends any>() {}

function commented<T /*a*/ extends /*b*/ any /*c*/>() {}

const A = <T extends unknown>() => {}
const B = <T extends unknown = unknown>() => {}
