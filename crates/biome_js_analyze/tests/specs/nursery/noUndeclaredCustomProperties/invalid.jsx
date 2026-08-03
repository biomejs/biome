/* should generate diagnostics */
const invalid = <div style={{ color: "var(--missing-color)" }} />;
const invalidTemplate = <div style={{ color: `var(--missing-template-color)` }} />;
const invalidMultiple = <div style={{ color: "var(--first-missing)", background: "var(--second-missing)" }} />;
