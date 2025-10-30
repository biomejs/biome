import Component from "@glimmer/component";

export default class MyComponent extends Component {
	message = "Hello, Glimmer!";

	get formattedMessage() {
		return this.message.toUpperCase();
	}
}
