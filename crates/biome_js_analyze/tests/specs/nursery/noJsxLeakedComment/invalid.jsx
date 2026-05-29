/* should generate diagnostics */
const Invalid1 = () => <div>// invalid</div>;
const Invalid2 = () => <>// invalid</>;
const Invalid3 = () => <div>/* invalid */</div>;
const Invalid4 = () => (
	<div>
    // invalid
	</div>
);
const Invalid5 = () => (
	<div>
		abcdef
		/* invalid */
		foo
	</div>
);
const Invalid6 = () => (
	<div>
		{'abcdef'}
        // invalid
		{'foo'}
	</div>
);
const Invalid7 = () => <span>/*</span>;
