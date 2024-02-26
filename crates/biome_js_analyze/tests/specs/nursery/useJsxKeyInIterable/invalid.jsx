import React from "react";

[<Hello />, <Hello />, <Hello />];

[...[<Hello />, <Hello />], <Hello />];

[<Hello />, xyz ? <Hello />: <Hello />, <Hello />];

[<></>, <></>, <></>];

data.map(x => <Hello>{x}</Hello>);

data.map(x => <>{x}</>);

data.forEach(x => data1.push(<>{x}</>));

Array.from([1, 2, 3], (x) => <Hello>{x}</Hello>);

Array.from([1, 2, 3], (x) => {
	return <Hello>{x}</Hello>
});

Array.from([1, 2, 3], (x) => {
	const xyz = <h1></h1>

	if (a)
		return (
			<Hello>
				<div>{x}</div>
			</Hello>
		);

	return <Hello>{x}</Hello>
});

[React.createElement("h1"), React.createElement("h1"), React.createElement("h1")];

data.map(c => React.createElement("h1"));

React.Children.map(c => React.cloneElement(c));
