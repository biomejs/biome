// These are valid type arguments
<A extends B>() => {};
<A=string>() => {};
<A, B>() => {};
<A extends B<C>>() => {}
