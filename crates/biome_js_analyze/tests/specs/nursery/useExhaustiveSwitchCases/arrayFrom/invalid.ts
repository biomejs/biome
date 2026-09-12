// should generate diagnostics
import { Player } from "./player.js";
import { players as importedPlayers } from "./players.js";

const players = Array.from({ length: 5 }, () => new Player("running"));

function updatePlayers() {
	switch (players[0].state) {
		case "running":
			break;
	}

	switch (importedPlayers[0].state) {
		case "running":
			break;
	}
}
