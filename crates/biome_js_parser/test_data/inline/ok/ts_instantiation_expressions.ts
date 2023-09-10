let f1 = fx<string>;
let f2 = fx<string, number>;
let f3 = fx['test']<string>;
const a2 = f.g<number>;  // () => number
const a5 = f['g']<number>;  // () => number
const a7 = (f<number>)['g'];
const a6 = f<number>['g'];  // type Error
const b2 = f?.<number>();
const b3 = f<number>?.();
const b4 = f<number>?.<number>();  // Type Error, expected no type arguments
const x1 = f<true>
(true);
const x2 = f<true>
true;
const x3 = f<true>;
true;
(f<T>)<K>;
(f<T>)<K>();
(f<T>)<K>?.();
(a?.f<T>)<K>();
new (a<T>)<K>();
f<<T>() => T>?.();
f?.<<T>() => T>();
f<x> ? g<y> : h<z>;
[f<x>];
{ f<x> }
