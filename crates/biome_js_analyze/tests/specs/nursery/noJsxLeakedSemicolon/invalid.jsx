/* should generate diagnostics */
const Invalid1 = () => {
	return (
		<div>
			<div />;
		</div>
	);
}

const Invalid2 = () => {
	return (
		<div>
			<Component>
				<div />
			</Component>;
		</div>
	);
}

const Invalid3 = () => (
	<div>
		<Component />;
	</div>
)

const Invalid4 = () => {
	return (
		<>
			<div />;
		</>
	);
}

const Invalid5 = () => {
	return (
		<>
			<Component>
				<div />
			</Component>;
		</>
	);
}
