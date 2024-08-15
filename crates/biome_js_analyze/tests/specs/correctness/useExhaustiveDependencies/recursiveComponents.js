// issue: https://github.com/biomejs/biome/issues/3212

import { useMemo } from "react";

function MyRecursiveComponent1() {
	const children = useMemo(() => <MyRecursiveComponent1 />, []);
	return <div>{children}</div>;
}

export function MyRecursiveComponent2() {
	const children = useMemo(() => <MyRecursiveComponent2 />, []);
	return <div>{children}</div>;
}

export default function MyRecursiveComponent3() {
	const children = useMemo(() => <MyRecursiveComponent3 />, []);
	return <div>{children}</div>;
}

const MyRecursiveComponent4 = () => {
	const children = useMemo(() => <MyRecursiveComponent4 />, []);
	return <div>{children}</div>;
}

const MyRecursiveComponent5 = function() {
	const children = useMemo(() => <MyRecursiveComponent5 />, []);
	return <div>{children}</div>;
}

const MyRecursiveComponent6 = function f() {
	const children = useMemo(() => <MyRecursiveComponent6 />, []);
	return <div>{children}</div>;
}
