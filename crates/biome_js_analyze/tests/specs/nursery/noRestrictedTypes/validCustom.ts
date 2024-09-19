let a: String;

let e: Object;

let b: { c: String };

function foo(a: String) {}

'a' as String;

class Foo<T = String> extends Bar<String> implements Baz<Object> {
  constructor(foo: String | Object) {}

  exit(): Array<String> {
    const foo: String = 1 as String;
  }
}

let baz: [boolean, Boolean] = [true, false];

let z = true as Boolean;

type Props = {};

let fn: Function = () => true

const str: String = 'foo';

const bool: Boolean = true;

const num: Number = 1;

const symb: Symbol = Symbol('foo');

const bigInt: BigInt = 1n;

const lowerObj: Object = {};

const capitalObj: Object = { a: 'string' };

const curly1: {

} = 1;

const curly2: {} = { a: 'string' };
