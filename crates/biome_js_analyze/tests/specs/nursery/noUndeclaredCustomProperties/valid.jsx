/* should not generate diagnostics */
import "./theme.css";

export const Imported = () => <div style="color: var(--theme-color)" />;
export const Inline = () => (
	<div style="--inline-color: red; color: var(--inline-color)" />
);
export const Dynamic = () => <div style={{ color: "var(--dynamic-color)" }} />;
