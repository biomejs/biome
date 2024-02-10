import PlaygroundLoader from "@/playground/PlaygroundLoader";
import React from "react";
import ReactDom from "react-dom";

ReactDom.render(
	<React.StrictMode>
		<PlaygroundLoader />
	</React.StrictMode>,
	document.getElementById("root"),
);
