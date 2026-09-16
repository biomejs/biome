type State = "running" | "jumping" | "ducking";

export class Player {
	state: State;
	constructor(state: State) {
		this.state = state;
	}
}
