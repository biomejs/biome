export class Player {
	state: "running" | "jumping" | "ducking";
	constructor(state: "running" | "jumping" | "ducking") {
		this.state = state;
	}
}
