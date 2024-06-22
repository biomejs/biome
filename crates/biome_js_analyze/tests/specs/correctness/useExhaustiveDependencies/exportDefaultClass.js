import { Component, useMemo } from "react";

export default class ClassComponent extends Component {
  render() {
    return <h2>Hi, I am a Class component!</h2>;
  }
}

function FunctionComponent() {
	const children = useMemo(() => <ClassComponent />, []);
	return <div>{children}</div>;
}
