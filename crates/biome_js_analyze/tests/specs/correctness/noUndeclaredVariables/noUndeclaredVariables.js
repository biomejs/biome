// invalid
foobar;
function f() {
    lorem;
}
assignment = "value";
<Missing />;

// valid
document;
navigator;
new ArrayBuffer();
new AggregateError();
// Temporal (TC39 Stage 4, shipped in Chrome/Firefox) should not be flagged
Temporal.Now.instant();
