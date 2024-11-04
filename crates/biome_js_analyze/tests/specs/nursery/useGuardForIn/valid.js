for (var x in o);
for (var x in o) {}
for (var x in o) if (x) f();
for (var x in o) { if (x) { f(); } }
for (var x in o) { if (x) continue; f(); }
for (var x in o) { if (x) { continue; } f(); }
for (var x in o) { if(Object.hasOwn(o, x)) f(); }