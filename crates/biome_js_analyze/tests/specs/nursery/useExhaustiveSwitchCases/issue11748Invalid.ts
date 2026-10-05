// should generate diagnostics
import { players } from "./issue11748Players.js";

function updatePlayers() {
	switch (players[0].state) {
		case "running":
			break;
	}
}
