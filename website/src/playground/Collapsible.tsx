import { useState } from "react";
import { classnames } from "@/playground/utils";

interface Props {
	className?: string;
	heading: string | JSX.Element;
	children: JSX.Element;
}

export default function Collapsible(props: Props) {
	const [visible, setVisible] = useState(true);

	function onClick() {
		setVisible(!visible);
	}

	const className = classnames(!visible && "collapsed", props.className);

	return (
		<div className={classnames("collapsible-container", className)}>
			<h2
				onClick={onClick}
				onKeyUp={onClick}
				className={classnames("collapsible", className)}
			>
				{props.heading}
			</h2>
			{visible && <div className="collapsible-content">{props.children}</div>}
		</div>
	);
}
