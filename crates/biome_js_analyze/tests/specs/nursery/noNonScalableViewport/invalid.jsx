/* should generate diagnostics */
const Invalid = () => (
	<>
		<meta name="viewport" content="width=device-width, user-scalable=no" />
		<meta name="viewport" content="user-scalable=no"></meta>
		<meta name="viewport" content="user-scalable = no, width=device-width" />
		<meta name={"VIEWPORT"} content={"USER-SCALABLE=NO"} />
		<meta name="viewport" content={`user-scalable=no`} />
	</>
);
