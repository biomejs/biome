/* should not generate diagnostics */

type Handler = () => void;

export function mprisMethods(player: { next(): void; stop(): void }): Record<string, Handler> {
	return {
		Next: () => player.next(),
		Stop: function () {
			player.stop();
		},
	};
}
