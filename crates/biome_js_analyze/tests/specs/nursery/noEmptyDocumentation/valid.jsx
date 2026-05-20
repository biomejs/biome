/* should not generate diagnostics */

const Valid1 = (
	<div>
		{/* valid */}
		<input />
	</div>
);

const Valid2 = (
	<div>
		{/* valid */}
		{/* valid */}
		<input />
	</div>
);

const Valid3 = (
	<div>
		<input /> {/* valid */}
	</div>
);
