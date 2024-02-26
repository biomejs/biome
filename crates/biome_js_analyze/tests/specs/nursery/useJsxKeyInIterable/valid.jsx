/* should not generate diagnostics */

import React from "react";

[<Hello key="first" />, <Hello key="second" />, <Hello key="third" />];

[...[<Hello key="first" />, <Hello key="second" />], <Hello key="third" />];

[<Hello key="first" />, xyz ? <Hello key="second"/>: <Hello key="second" />, <Hello key="third" />];

[<React.Fragment key="first"></React.Fragment>, <React.Fragment key="second"></React.Fragment>, <React.Fragment key="third"></React.Fragment>];

data.map(x => <Hello key={x}>{x}</Hello>);

data.map(x => <React.Fragment key={x}>{x}</React.Fragment>);

data.forEach(x => data1.push(<React.Fragment key={x}>{x}</React.Fragment>));

Array.from([1, 2, 3], (x) => <Hello key={x}>{x}</Hello>);

Array.from([1, 2, 3], (x) => {
	return <Hello key={x}>{x}</Hello>
});

Array.from([1, 2, 3], (x) => {
	const xyz = <h1 key={x}></h1>

	if (a)
		return (
			<Hello key={x}>
				<div>{x}</div>
			</Hello>
		);

	return <Hello key={x}>{x}</Hello>
});

[React.createElement("h1", {key}), React.createElement("h1", {key: "second"}), React.createElement("h1", {key: third})];

data.map(c => React.createElement("h1", {key}));

React.Children.map(c => React.cloneElement(c, {key: c}));
