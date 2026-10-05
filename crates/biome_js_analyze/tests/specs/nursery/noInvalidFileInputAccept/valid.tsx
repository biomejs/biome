/* should not generate diagnostics */
const Valid = () => (
	<input type="file" accept={"image/png" as const} />
);
