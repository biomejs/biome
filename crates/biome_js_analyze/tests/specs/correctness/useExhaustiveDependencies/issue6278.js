import React from "react";

const Comp = (myFn) => {
	React.useCallback(() => {
		myFn(true);
		myFn(false);
	}, []);
};
