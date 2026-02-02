// should not generate diagnostics

// Already using ??
declare const maybeStr: string | null;
const a = maybeStr ?? 'default';

// && operator - not our concern
declare const x: string | null;
declare const y: string;
const b = x && y;

// Non-nullish types
declare const definiteString: string;
const c = definiteString || 'fallback';

declare const definiteNumber: number;
const d = definiteNumber || 0;

declare const definiteBoolean: boolean;
const e = definiteBoolean || false;

// Conditional test positions (ignored by default)
declare const cond: string | null;

if (cond || 'fallback') {
  console.log('in if');
}

while (cond || 'fallback') {
  break;
}

for (; cond || 'fallback'; ) {
  break;
}

do {
  break;
} while (cond || 'fallback');

const result = (cond || 'fallback') ? 'yes' : 'no';

// Non-nullish union types
declare const strOrNum: string | number;
const f = strOrNum || 'default';

declare const obj: { prop: string };
const g = obj.prop || 'default';

function getString(): string {
  return 'value';
}
const h = getString() || 'default';
