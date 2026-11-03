/* should generate diagnostics */
const a1 = { ...((foo as object /* keep */) ?? {}) };
const a2 = { ...(foo! || {}) };
