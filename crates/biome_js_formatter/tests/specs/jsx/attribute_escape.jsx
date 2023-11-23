const a = () => {
	const a = "\test";
	const b = "\\test";
	const c = "\\\test";
	const d = "\\\\test";

	return (
		<>
			<input name="pin" type="text" pattern="\d{4,4}" required />
			<input name="\\pin" type="text" pattern="\d{4,4}" required />
			<input name="\\\pin" type="text" pattern="\d{4,4}" required />
			<input name="\\\\pin" type="text" pattern="\d{4,4}" required />
		</>
	);
};
