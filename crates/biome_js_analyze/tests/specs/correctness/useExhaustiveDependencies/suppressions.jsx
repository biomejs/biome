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

function SingleInstanceSuppressionWrong({a, b}) {
	// biome-ignore lint/correctness/useExhaustiveDependencies(b): test
	useEffect(() => {}, [a]);
}

function SingleInstanceSuppressionDuplicate({a, b}) {
	// biome-ignore lint/correctness/useExhaustiveDependencies(b): test
	// biome-ignore lint/correctness/useExhaustiveDependencies(b): test
	useEffect(() => {}, [b]);
}

function SingleInstanceSuppressionNotEnough({a, b}) {
	// biome-ignore lint/correctness/useExhaustiveDependencies(a): test
	useEffect(() => {}, [a, b]);
}

function SingleInstanceSuppressionNotEnough2({a, b}) {
	// biome-ignore lint/correctness/useExhaustiveDependencies(b): test
	useEffect(() => {}, [a, b]);
}

function MultiInstanceSuppressionSomeWrong({a, b, c}) {
	// biome-ignore lint/correctness/useExhaustiveDependencies(a): test
	// biome-ignore lint/correctness/useExhaustiveDependencies(c): test
	useEffect(() => {}, [a, b]);
}

function MultiInstanceSuppressionAllWrong({a, b, c, d}) {
	// biome-ignore lint/correctness/useExhaustiveDependencies(c): test
	// biome-ignore lint/correctness/useExhaustiveDependencies(d): test
	useEffect(() => {}, [a, b]);
}
