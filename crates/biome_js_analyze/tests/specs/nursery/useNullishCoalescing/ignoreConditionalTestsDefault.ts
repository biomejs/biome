/* should generate diagnostics */

declare const cond: string | null;
declare const other: string;

// `if` test: not reported
if (cond || 'fallback') {
  console.log('in if');
}

// `while` test: not reported
while (cond || 'fallback') {
  break;
}

// `for` test: not reported
for (; cond || 'fallback'; ) {
  break;
}

// `do-while` test: not reported
do {
  break;
} while (cond || 'fallback');

// Ternary test: not reported
const result = (cond || 'fallback') ? 'yes' : 'no';

// Through `!`, a logical operator, a ternary branch, or the last expression of a sequence: not reported
if (!(cond || 'fallback')) {}
if (other && (cond || 'fallback')) {}
if (other ? cond || 'fallback' : other) {}
if ((other, cond || 'fallback')) {}

// Through `||` or `??`: not reported
if (cond || other || 'fallback') {}
if (other || (cond || 'fallback')) {}
if ((cond || other) ?? 'fallback') {}
if (cond ?? (other || 'fallback')) {}

// `||=` in a test: not reported
declare let assigned: string | null;
if ((assigned ||= 'fallback')) {}
while ((assigned ||= 'fallback')) {}
const assignedResult = (assigned ||= 'fallback') ? 'yes' : 'no';

// A truthiness ternary in a test: not reported
if (cond ? cond : other) {}

// Through a call argument, a unary `+`, or a function body: reported
declare function check(value: unknown): unknown;
if (check(cond || 'fallback')) {}
if (+(cond || 'fallback')) {}
if (() => cond || 'fallback') {}
if (
  function weird() {
    return cond || 'fallback';
  }
) {}

// Earlier expression of a sequence: reported
if ((cond || 'fallback', other)) {}

// Comparison ternary in a test: reported
if (cond != null ? cond : other) {}

// `for` initializer and update, and `switch` discriminant: reported
for (cond || 'fallback'; ; ) {
  break;
}
for (; ; cond || 'fallback') {
  break;
}
switch (cond || 'fallback') {
  default:
}

// Operand of a comparison or `typeof`, or the branch of a ternary that is not a test: reported
if ((cond || 'fallback') === other) {}
if (typeof (cond || 'fallback')) {}
const notATest = other ? cond || 'fallback' : other;

// Double negation: not reported
if (!!(cond || 'fallback')) {}
