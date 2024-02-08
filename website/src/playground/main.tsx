import React from "react";
import ReactDom from "react-dom";
import PlaygroundLoader from "@/playground/PlaygroundLoader";

ReactDom.render(
	<React.StrictMode>
		<PlaygroundLoader />
	</React.StrictMode>,
	document.getElementById("root"),
);
