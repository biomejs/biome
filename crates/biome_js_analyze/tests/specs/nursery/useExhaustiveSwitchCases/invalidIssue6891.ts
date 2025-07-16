type State = "running" | "jumping" | "ducking"

class Player {
  state: State
  constructor(state: State) {
    this.state = state
  }
}

export function updatePlayers(players: Player[]) {
  switch(players[0].state) {
    case "running":
      break;
    // Here Biome should error out saying that the "jumping" and "ducking" cases are not handled
  }
}
