// should generate diagnostics
import type { Player } from "./issue12073Player.js";

class GameController {
	players: Player[];

	updatePlayers() {
		for (const [i, player] of this.players.entries()) {
			switch (player.state) {
				case "running":
					break;
			}
		}
	}

	updatePlayersByIndex() {
		for (const [i] of this.players.entries()) {
			switch (this.players[i].state) {
				case "running":
					break;
			}
		}
	}
}
