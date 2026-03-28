/* should generate diagnostics */

// Record with array values sorted without a comparator
const groups: Record<string, number[]> = {};
groups["a"].sort();
groups["b"].toSorted();
