/* should not generate diagnostics */
import "./styles.css";

const fromObject = <div style={{ "--local-color": "red", color: "var(--local-color)" }} />;
const fromStylesheet = <div style={{ color: "var(--global-color)" }} />;
const dynamic = <div style={styles} />;
const spread = <div style={{ ...styles, color: "var(--possibly-spread-color)" }} />;
const dynamicKey = <div style={{ [propertyName]: "red", color: "var(--possibly-computed-color)" }} />;
const computedKey = <div style={{ ["var(--not-a-reference)"]: "red" }} />;
const staticTemplate = <div style={{ "--template-color": "red", color: `var(--template-color)` }} />;
const escapedLeadingHyphens = <div style={{ "--escaped-leading": "red", color: "var(\\2d \\2d escaped-leading)" }} />;
