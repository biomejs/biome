// These are valid type arguments
<A,>() => {};
<const A,>() => {};
<A extends B>() => {};
<A=string>() => {};
<A, B>() => {};
<A extends B<C>>() => {};
