import Component from "@glimmer/component";

interface Signature {
	Args: {
		name: string;
	};
}

export default class Greeting extends Component<Signature> {
	get greeting(): string {
		return `Hello, ${this.args.name}!`;
	}

	<template>
    <div class="greeting">
      <h1>{{this.greeting}}</h1>
    </div>
  </template>;
}
