function Component2() {
	const str = "str";
	// should trigger
	return <>{str}</>;
}

const obj = {
	// should not trigger
	element: <>test</>,
};
