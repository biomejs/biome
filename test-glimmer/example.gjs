import Component from "@glimmer/component";

export default class MyComponent extends Component {
	message = "Hello, Glimmer!";

	get formattedMessage() {
		return this.message.toUpperCase();
	}

	<template>
    <div class="greeting">
      <h1>{{this.formattedMessage}}</h1>
      <p>Welcome to Glimmer components!</p>
    </div>
  </template>;
}
