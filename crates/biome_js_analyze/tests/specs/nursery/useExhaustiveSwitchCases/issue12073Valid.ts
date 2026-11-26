// should not generate diagnostics
import type { Player } from "./issue12073Player.js";

declare const players: Player[];

for (const [i, player] of players.entries()) {
	switch (player.state) {
		case "running":
		case "jumping":
		case "ducking":
			break;
	}
	switch (players[i].state) {
		case "running":
		case "jumping":
		case "ducking":
			break;
	}
}
