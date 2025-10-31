import Component from "@glimmer/component";

interface Signature {
	Args: {
		title: string;
	};
}

export default class TypedComponent extends Component<Signature> {
	<template>
    <h1>{{@title}}</h1>
  </template>

	get computedValue(): string {
		return "TypeScript works!";
	}
}
