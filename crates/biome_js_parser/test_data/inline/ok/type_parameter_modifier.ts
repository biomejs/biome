type Foo<in T> = {}
type Foo<out> = {}
type Foo<out T> = {}
type Foo<in out> = {}
type Foo<out out> = {}
type Foo<in out out> = {}
type Foo<in X, out Y> = {}
type Foo<out X, in Y> = {}
type Foo<out X, out Y extends keyof X> = {}
class Foo<in T> {}
class Foo<out T> {}
export default class Foo<in T> {}
class Foo<out T> {}
interface Foo<in T> {}
interface Foo<out T> {}
declare class Foo<in T> {}
declare class Foo<out T> {}
declare interface Foo<in T> {}
declare interface Foo<out T> {}
function a<const T>() {}
function b<const T extends U>() {}
function c<T, const U>() {}
declare function d<const T>();
<T>() => {};
<const T>() => {};
(function <const T>() {});
(function <const T extends U>() {});
(function <T, const U>() {});
class A<const T> {}
class B<const T extends U> {}
class C<T, const U> {}
class D<in const T> {}
class E<const in T> {}
class F<in const out T> {}
(class <const T> {});
(class <const T extends U> {});
(class <T, const U> {});
(class <in const T> {});
(class <const in T> {});
class _ {
  method<const T>() {}
  method<const T extends U>() {}
  method<T, const U>() {}
}
declare module a {
  function test<const T>(): T;
}
const obj = {
  a<const T>(b: any): b is T { return true; }
}
