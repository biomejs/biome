// should generate diagnostics
type State = "running" | "jumping" | "ducking";

class Player {
	state: State;
	constructor(state: State) {
		this.state = state;
	}
}

const players = Array.from({ length: 5 }, () => new Player("running"));
switch (players[0].state) {
	case "running":
		break;
}

const from = Array.from;
const mapped = from([1, 2], (value: number, index: number) => new Player("running"), {});
switch (mapped[0].state) {
	case "running":
		break;
}

declare const source: readonly Player[];
const copied = Array.from(source);
switch (copied[0].state) {
	case "running":
		break;
}

const contextual = Array.from(source, player => player.state);
switch (contextual[0]) {
	case "running":
	case undefined:
		break;
}

declare const iterable: Iterable<Player>;
const fromIterable = Array.from(iterable);
switch (fromIterable[0].state) {
	case "running":
		break;
}
