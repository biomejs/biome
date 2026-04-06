// Test cases for comment trivia preservation around if-statement autofix

declare let a: { x: string } | null;
declare function makeFoo(): { x: string };

// Leading comment should be preserved
if (a == null) {
    a = makeFoo();
}

// Inline comment in condition
declare let b: { x: string } | null;
if (b == null /* check */) {
    b = makeFoo();
}

// Comment before assignment
declare let c: { x: string } | null;
if (c == null) {
    // assignment comment
    c = makeFoo();
}

// Bare expression with leading comment
declare let d: { x: string } | null;
// bare leading
if (!d) d = makeFoo();
