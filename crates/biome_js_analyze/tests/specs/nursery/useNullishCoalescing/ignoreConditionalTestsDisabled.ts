/* should generate diagnostics */

declare const a: string;
declare const b: number;
declare const c: boolean;

// `if` test
if (null || a) {
  console.log(a);
}

// `while` test
while (null || b) {
  break;
}

// `for` test
for (; null || c; ) {
  break;
}

// `do-while` test
do {
  break;
} while (null || 'fallback');

// Ternary test
const result = (null || 42) ? 'yes' : 'no';
