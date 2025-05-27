import React from "react";

[<Hello />, <Hello />, <Hello />];

[...[<Hello />, <Hello />], <Hello />];

[<Hello />, xyz ? <Hello />: <Hello />, <Hello />];

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

(<h1>{data.map(c => (<h1></h1>))}</h1>)

(<h1>{data.map(c => {return (<h1></h1>)})}</h1>)

[].map((item) => {
	return item.condition ? <div /> : <div>foo</div>;
});

[].map((item) => {
	const x = 5;
	const div = <div>{x}</div>;
	return div;
});

[].map(function(item) {
	const x = 5;
	const div = <div>{x}</div>;
	return div;
});

data.map((item) => <React.Fragment><p>{item}</p></React.Fragment>)

data.map((x) => {
	if (x.type === 'string') {
		return <div>{x.value}</div> // no key
	}
	return <div>{x.value}</div> // no key
})

data.map((x) => {
	if (x.type === 'string') {
		return <div>{x.value}</div> // no key
	}
	return <div key={x.value}>{x.value}</div>
})

data.map((x) => {
	if (x.type === 'string') {
		return <div>{x.value}</div> // no key
	} else {
		return <div>{x.value}</div> // no key
	}
})

data.map((x) => {
	switch (x.type) {
		case 'string':
			return <div>{x.value}</div> // no key
		case 'number':
			return <div>{x.value}</div> // no key
		default:
			return <div key={x.value}>{x.value}</div>
	}
})

data.map((x) => {
	switch (x.type) {
		case 'string':
			return <div>{x.value}</div> // no key
		case 'number':
			return <div>{x.value}</div> // no key
		default:
			return <div>{x.value}</div> // no key
	}
})
