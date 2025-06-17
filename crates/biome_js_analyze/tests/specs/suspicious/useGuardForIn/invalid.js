for (var x in o) { if (x) { f(); continue; } g(); }
for (var x in o) { if (x) { continue; f(); } g(); }
for (var x in o) { if (x) { f(); } g(); }
for (var x in o) { if (x) f(); g(); }
for (var x in o) { foo() }
for (var x in o) foo();