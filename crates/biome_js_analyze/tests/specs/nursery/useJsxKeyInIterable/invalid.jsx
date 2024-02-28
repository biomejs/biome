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

[React.createElement("h1"), React.createElement("h1"), React.createElement("h1")];

data.map(c => React.createElement("h1"));

React.Children.map(c => React.cloneElement(c));

(<h1>{data.map(x => <h1>{x}</h1>)}</h1>)

(<h1>{[<h1></h1>, <h1></h1>, <h1></h1>]}</h1>)

(<h1>{[<h1></h1>, xyz, abc? <h2></h2>: bcd]}</h1>)

(<h1>{data.map(c => <h1></h1>)}</h1>)

(<h1>{data.map(c => xyz)}</h1>)
