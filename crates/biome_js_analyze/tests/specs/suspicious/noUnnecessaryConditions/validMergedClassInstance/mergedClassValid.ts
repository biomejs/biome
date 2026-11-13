/* should not generate diagnostics */

export class Settings {
	theme = "dark";
}

export namespace Settings {
	export const defaults = new Settings();
}
