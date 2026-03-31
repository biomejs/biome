/* should generate diagnostics */

function defaultCall<T = number>() {}
defaultCall<number>();

function inferredCall<T>(value: T) {}
inferredCall<number>(10);

declare const x: number;
inferredCall<number>(x);

function inferredSecond<T, U>(first: T, second: U) {}
inferredSecond<number, number>(10, 10);

function tagged<T = number>(templates: TemplateStringsArray, value: T) {}
tagged<number>`${1}`;

class Box<T = number> {}
new Box<number>();

class Wrapped<T> {
  constructor(value: T) {}
}
new Wrapped<number>(10);

class Base<T = string> {}
class Derived extends Base<string> {}

interface Shape<T = string> {}
class Impl implements Shape<string> {}

type Alias<T = Map<string, string>> = T;
type Value = Alias<Map<string, string>>;

function withDefaultTail<T, U = string>(first: T, second: U) {}
withDefaultTail<number, string>(1, "value");
