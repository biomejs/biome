// should not generate diagnostics
import type { Player } from "./issue11730Player.js";

class GameController {
	players: Player[];

	updatePlayers() {
		for (const [i, player] of this.players.entries()) {
			switch (player.state) {
				case "running":
				case "jumping":
				case "ducking":
					break;
			}

			switch (this.players[i].state) {
				case "running":
				case "jumping":
				case "ducking":
					break;
			}
		}
	}
}
