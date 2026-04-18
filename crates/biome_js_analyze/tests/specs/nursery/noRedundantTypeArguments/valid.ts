/* should not generate diagnostics */

function defaultCall<T = number>() {}
defaultCall<string>();
defaultCall();

function inferredCall<T>(value: T) {}
declare const unknownValue: any;
inferredCall<string>(unknownValue);

function inferredSecond<T, U>(first: T, second: U) {}
inferredSecond<number, string>(10, unknownValue);

function tagged<T = number>(templates: TemplateStringsArray, value: T) {}
tagged<string>`${1}`;

class Box<T = number> {}
new Box<string>();

class Wrapped<T> {
  constructor(value: T) {}
}
new Wrapped<number[]>([]);

class Base<T = string> {}
class Derived extends Base<number> {}

interface Shape<T = string> {}
class Impl implements Shape<number> {}

type Alias<T = Map<string, string>> = T;
type Value = Alias<Map<string, number>>;

function withDefaultTail<T, U = string>(first: T, second: U) {}
withDefaultTail<number, number>(1, unknownValue);
