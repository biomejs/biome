/* should generate diagnostics */
for (var i = !(a === b in c); ;) {}
for (let ok = !("k" in o === false); ;) {}
for (var f = () => !(a === b in c); ;) {}
for (let x = !(a === b); ;) {}
for (const key in !(a === b) ? x : y) {}
