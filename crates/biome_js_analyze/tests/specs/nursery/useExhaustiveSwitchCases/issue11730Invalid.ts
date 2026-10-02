// should generate diagnostics
import type { Player } from "./issue11730Player.js";

class GameController {
	players: Player[];

	updatePlayers() {
		for (const player of this.players) {
			switch (player.state) {
				case "running":
					break;
			}
		}
	}
}
