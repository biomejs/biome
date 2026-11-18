/* should generate diagnostics */
const selected = new Set([1, 2]); const other = new Set([2, 3]); const records = new Map([[2, "value"]]); const view = <div>{selected.union(new Set(other))}</div>
