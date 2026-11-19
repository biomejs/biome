/* should not generate diagnostics */
const dynamicName = "viewport";
const dynamicContent = "user-scalable=no";

const Valid = () => (
	<>
		<div name="viewport" content="user-scalable=no" />
		<Meta name="viewport" content="user-scalable=no" />
		<meta name="other" content="user-scalable=no" />
		<meta name="viewport" />
		<meta content="user-scalable=no" />
		<meta name="viewport" content="user-scalable=yes" />
		<meta name="viewport" content="user-scalable=nope" />
		<meta name="viewport" content="not-user-scalable=no" />
		<meta name={dynamicName} content="user-scalable=no" />
		<meta name="viewport" content={dynamicContent} />
		<meta {...props} />
	</>
);
