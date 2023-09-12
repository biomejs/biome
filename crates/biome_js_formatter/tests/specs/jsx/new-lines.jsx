

x = (
	<div>
		<div>First</div>,<div>Second</div>
	</div>
);

x = (
	<div>
		<div>First</div>,
		<div>Second</div>
	</div>
);

x = (
	<div>
		<div>First</div>
		,<div>Second</div>
	</div>
);

function Component() {
	return (
		<>
			<span>text</span>.<br />
		</>
	);
}

let myDiv1 = ReactTestUtils.renderIntoDocument(
	<div1>
		<div key="theDog" className="dog" />,<di key="theBird" className="bird" />
	</div1>
);


let myDiv2 = ReactTestUtils.renderIntoDocument(
	<div1>
		<div key="theDog" className="dog" />
		,<di key="theBird" className="bird" />
	</div1>
);

let myDiv3 = ReactTestUtils.renderIntoDocument(
	<div1>
		<div key="theDog" className="dog" />,
		<di key="theBird" className="bird" />
	</div1>
);
