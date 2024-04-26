foo.length && true
Boolean(foo.length == 0);
Boolean(foo.length != 0);



!Boolean(!(Boolean(0 != foo.length)));
// !(!(Boolean(Boolean(foo.length))))
// foo.length && true
// foo.length!=0
// if (/** A */foo.size /** B */) {}
// Boolean(foo.length)
// new Boolean(foo.length)
// !foo.length
// !this.length
// true ? foo.length : 0
// true ? foo.length == 0 : 0

// !!!(!!foo.length)
