/* should not generate diagnostics */
function getValue() {
	return Math.random() > 0.5;
}

export default function MyComponent() {
	return <button onClick={getValue() ? getValue : undefined} />;
}

const Component1 = () => {
	return isPending ? loader : userResults;
};

const Component2 = () => {
	return someRandomId && !someRandomBooleanValue ? <ContentHere /> : <NotFound />;
};

// Ignore Attribute
const Component3 = () => {
	return (
		<NoErrorWithAttribute
			error={firstBool && secondBool}
			coerceError={!!(firstBool && secondBool)}
			orderId={isChecked ? orderId : undefined}
		/>
	);
};

const Component4 = () => {
	return (
		<Nested>
			<SecondNested>{userId ? 1 : null} </SecondNested>
			<SecondNested>{userId ? 1 : undefined} </SecondNested>
		</Nested>
	);
};

const Component5 = () => {
	const [isOpen, setIsOpen] = useState(false);
	const [isReady] = useState<boolean>();
	return (
		<div>
			{isOpen && <Modal />}
			{isReady && <Modal />}
			{(isOpen && <Modal />)}
		</div>
	);
};

const Component6 = ({ fallback }) => {
	return <div>{isOpen ? <Modal /> : fallback}</div>;
};
