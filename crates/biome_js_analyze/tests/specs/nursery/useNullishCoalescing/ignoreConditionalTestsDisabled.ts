// should generate diagnostics

declare const a: string;
declare const b: number;
declare const c: boolean;

if (null || a) {
  console.log(a);
}

while (null || b) {
  break;
}

for (; null || c; ) {
  break;
}

do {
  break;
} while (null || 'fallback');

const result = (null || 42) ? 'yes' : 'no';
