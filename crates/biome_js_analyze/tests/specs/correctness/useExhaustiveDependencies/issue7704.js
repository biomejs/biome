import { useEffect } from "react";

function InvalidComponent(props) {
	useEffect(() => {
		props.foo();
	}, []);
}

function InvalidComponent2(props) {
	useEffect(() => {
		props.foo();
	}, [props.foo]);
}

function ValidComponent(props) {
	useEffect(() => {
		props.foo();
	}, [props]);
}
