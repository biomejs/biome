/* should not generate diagnostics */
import { useEffect } from "react";

function FullSuppression({a, b}) {
	// biome-ignore lint/correctness/useExhaustiveDependencies: test
	useEffect(() => {}, [a, b]);
}

function SingleInstanceSuppression({a, b}) {
	// biome-ignore lint/correctness/useExhaustiveDependencies(a): test
	useEffect(() => {}, [a]);
}

function MultiInstanceSuppression({a, b}) {
	// biome-ignore lint/correctness/useExhaustiveDependencies(a): test
	// biome-ignore lint/correctness/useExhaustiveDependencies(b): test
	useEffect(() => {}, [a, b]);
}
