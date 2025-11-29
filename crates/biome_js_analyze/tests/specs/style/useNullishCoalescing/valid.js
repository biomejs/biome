/* should not generate diagnostics */

// Already using ??
const value = x ?? 'default';

// Test positions (with default option ignoreConditionalTests: true)
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

// UNSAFE DEFAULTS - should NOT report

// Numeric literals (0 is falsy!)
const count = n || 0;
const amount = value || 1;
const index = i || -1;

// Empty string (is falsy!)
const str = input || '';
const text = msg || "";

// Boolean literals (false is falsy!)
const flag = option || false;
const enabled = setting || true;

// null/undefined (are falsy!)
const nullable = x || null;
const undef = y || undefined;

// Identifiers (unknown type)
const fallback = primary || secondary;
const data = cached || fetched;

// Call expressions (could return anything)
const result2 = cached || getData();
const items = list || fetchItems();

// BOOLEAN CONTEXTS - should NOT report

// Boolean() call
const bool1 = Boolean(x || y);

// Double negation
const bool2 = !!(a || b);

// Single negation
const bool3 = !(x || y);

// MIXED LOGICAL CHAINS - should NOT report

// Mixed with &&
const mixed1 = a || b && c;
const mixed2 = (a || b) && c;

// Mixed with ??
const mixed3 = a ?? b || c;

// Complex expressions
const complex = (a || b) === c;
const ternary = (x || y) ? a : b;
