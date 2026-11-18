/* should generate diagnostics */
a |= 0;
foo.a.b |= 0;
foo[0] |= 0;
foo[key].bar |= 0;
this.value |= 0;
a >>= 0;
a <<= 0;
a ^= 0;
a |= 0; // comment
[foo][0] ^= 0;
(foo) |= 0;
arr[i + 1] |= 0;
arr[a ? -b : c && d] |= 0;
class A extends B { m() { super.x |= 0; } }
foo[`key`] |= 0;
foo[`${key}`] |= 0;
foo[a?.b] |= 0;
