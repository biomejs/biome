import { useEffect, useState } from "react";

function MyComponent() {
	const [name, setName] = useState('');
	const deps = [name];

	useEffect(() => {
		console.log(name); // we can't verify `name` is in the dependencies array
	}, deps);
}
