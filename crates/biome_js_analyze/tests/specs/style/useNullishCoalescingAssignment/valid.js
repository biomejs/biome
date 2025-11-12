/* should not generate diagnostics */

// Already using ??=
x ??= 'default';

// Regular assignment
x = x || 'default'; // This is covered by useNullishCoalescing

// Other assignment operators
x += 1;
x -= 2;
x *= 3;
x /= 4;
x &&= true;
