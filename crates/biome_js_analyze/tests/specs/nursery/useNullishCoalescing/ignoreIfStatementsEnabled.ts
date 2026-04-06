// Tests for ignoreIfStatements: true
// These should NOT generate diagnostics for if-statement patterns

declare let a: { x: string } | null;
declare function makeFoo(): { x: string };

// Truthiness check - should be ignored
if (!a) {
    a = makeFoo();
}

// Loose null check - should be ignored
declare let b: { x: string } | null;
if (b == null) {
    b = makeFoo();
}

// || should still be reported even with ignoreIfStatements
declare const c: string | null;
const r1 = c || 'default';
