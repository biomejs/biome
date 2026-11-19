// should not generate diagnostics
import { players } from "./issue11748Players.js";

function updatePlayers() {
	switch (players[0].state) {
		case "running":
		case "jumping":
		case "ducking":
			break;
	}
}
