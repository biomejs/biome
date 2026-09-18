/* should generate diagnostics */
{var a = b = c;}
{var a = b = c = d;}
{let foo = bar = cee = 100;}
{a=b=c=d=e}
{a=b=c}
{a
=b
=c}
{var a = (b) = (((c)))}
{var a = ((b)) = (c)}
{var a = b = ( (c * 12) + 2)}
{var a =
((b))
 = (c)}
{a = b = '=' + c + 'foo';}
{a = b = 7 * 12 + 5;}
{const x = {};
const y = x.one = 1;}
{let a, b;a = b = 1}
{let x, y;x = y = 'baz'}
{const a = b = 1}
{class C { field = foo = 0 }}
