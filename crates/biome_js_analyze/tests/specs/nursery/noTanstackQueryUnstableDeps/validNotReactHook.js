/* should not generate diagnostics */
import { useEffect as useOtherEffect } from "some-library";
import { useCallback } from "not-react";
import { useQuery } from "@tanstack/react-query";

function useMemo(callback, dependencies) {
	return callback(dependencies);
}

function Component() {
	const query = useQuery({ queryKey: ["a"], queryFn });
	const React = { useEffect() {} };

	useOtherEffect(() => {}, [query]);
	useCallback(() => {}, [query]);
	useMemo(() => {}, [query]);
	React.useEffect(() => {}, [query]);
}
