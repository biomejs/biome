/* should not generate diagnostics */

declare const cond: string | null;

// `if` test
if (cond || 'fallback') {
  console.log('in if');
}

// `while` test
while (cond || 'fallback') {
  break;
}

// `for` test
for (; cond || 'fallback'; ) {
  break;
}

// `do-while` test
do {
  break;
} while (cond || 'fallback');

// Ternary test
const result = (cond || 'fallback') ? 'yes' : 'no';
