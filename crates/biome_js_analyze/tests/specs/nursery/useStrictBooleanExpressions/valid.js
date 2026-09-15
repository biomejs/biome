// should not generate diagnostics
if (true) {}
if (false) {}
if ("text") {}
if ("") {}
if (0) {}
if (1n) {}
if (NaN) {}
if (Boolean(null)) {}
const shortCircuit = true && {};
const fallback = false || [];
const coalesced = null ?? {};
