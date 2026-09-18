/* should not generate diagnostics */

// Record with array values sorted with a comparator
const groups: Record<string, number[]> = {};
groups["a"].sort((a, b) => a - b);
groups["b"].toSorted((a, b) => a - b);
