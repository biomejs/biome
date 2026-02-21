const obj = {
  name: "Sample Item",
  details: {
    description: "This is a nested object",
    status: "active"
  },
  id: "12345",
  tags: ["short", "array"],
  metadata: {
    created: "2024-01-01",
    updated: "2024-01-02"
  },
  count: 42,
  multiLineArray: [
    "item1",
    "item2",
    "item3"
  ]
};

// Edge case: object with content on same line as closing brace (non-formatted)
const edgeCase1 = {
  obj: {
    b: "" },
  obj2: {

  },
  a: 1
};

// Edge case: empty multi-line object
const edgeCase2 = {
  empty: {
  },
  a: 1
};

// Edge case: single-line nested object (should NOT be treated as nested)
const edgeCase3 = {
  nested: { a: 1, b: 2 },
  z: 1
};
