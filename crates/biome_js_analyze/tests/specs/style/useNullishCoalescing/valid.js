/* should not generate diagnostics */

// Already using ??
const value = x ?? 'default';

// Not in test position (with default option ignoreConditionalTests: true)
if (x || y) {
  console.log('test');
}

while (a || b) {
  break;
}

for (; x || y; ) {
  break;
}

// Ternary test position
const result = (condition || backup) ? 'yes' : 'no';

// && operator (not our concern)
const value2 = x && y;

// ?? operator (already correct)
const value3 = x ?? y;
