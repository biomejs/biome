import { Player } from "./player.js";

export const players = Array.from({ length: 5 }, () => new Player("running"));
