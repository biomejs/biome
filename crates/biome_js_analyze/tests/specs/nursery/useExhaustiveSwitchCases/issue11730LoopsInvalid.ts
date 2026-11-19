// should generate diagnostics
import type { Player } from "./issue11730Player.js";

class GameController {
	players: Player[];
	pendingPlayers: Promise<Player>[];

	updatePlayers() {
		for (const { state } of this.players) {
			switch (state) {
				case "running":
					break;
			}
		}
	}

	async updatePendingPlayers() {
		for await (const player of this.pendingPlayers) {
			switch (player.state) {
				case "running":
					break;
			}
		}
	}
}

function updatePlayers(players: Player[]) {
	for (const player of players) {
		switch (player.state) {
			case "running":
				break;
		}
	}
}
