import { StatusBarAlignment, StatusBarItem, ThemeColor, window } from "vscode";
import { State } from "vscode-languageclient";
import { LanguageClient } from "vscode-languageclient/node";
import { Commands } from "./commands";

/**
 * Enumeration of all the status the extension can display
 *
 * The string value of the enum is the ThemeIcon ID to be displayer in the status
 * bar item when this status is active
 *
 * See https://code.visualstudio.com/api/references/icons-in-labels#icon-listing
 * for a list of available icons
 */
enum Status {
	Pending = "refresh",
	Ready = "check",
	Inactive = "eye-closed",
	Warning = "warning",
	Error = "error",
}

export class StatusBar {
	private statusBarItem: StatusBarItem;

	private serverState: State = State.Starting;
	private isActive = false;
	private serverVersion = "";

	constructor() {
		this.statusBarItem = window.createStatusBarItem(
			"biome.status",
			StatusBarAlignment.Right,
			-1,
		);

		this.statusBarItem.name = "Biome";
		this.statusBarItem.command = Commands.ServerStatus;
		this.update();
	}

	public setServerState(client: LanguageClient, state: State) {
		this.serverState = state;

		if (state === State.Running) {
			this.serverVersion = client.initializeResult?.serverInfo?.version ?? "";
		} else {
			this.serverVersion = "";
		}

		this.update();
	}

	public setActive(isActive: boolean) {
		this.isActive = isActive;
		this.update();
	}

	private update() {
		let status: Status;
		if (this.serverState === State.Running) {
			if (this.isActive) {
				status = Status.Ready;
			} else {
				status = Status.Inactive;
			}
		} else if (this.serverState === State.Starting) {
			status = Status.Pending;
		} else {
			status = Status.Error;
		}

		this.statusBarItem.text =
			`$(${status}) Rome ${this.serverVersion}`.trimEnd();

		switch (status) {
			case Status.Pending: {
				this.statusBarItem.tooltip = "Rome is initializing ...";
				break;
			}
			case Status.Ready: {
				this.statusBarItem.tooltip = "Rome is active";
				break;
			}
			case Status.Inactive: {
				this.statusBarItem.tooltip =
					"The current file is not supported or ignored by Rome";
				break;
			}
			// @ts-expect-error Reserved for future use
			case Status.Warning: {
				this.statusBarItem.tooltip = undefined;
				break;
			}
			case Status.Error: {
				this.statusBarItem.tooltip = "Rome encountered a fatal error";
				break;
			}
		}

		switch (status) {
			case Status.Error: {
				this.statusBarItem.color = new ThemeColor(
					"statusBarItem.errorForeground",
				);
				this.statusBarItem.backgroundColor = new ThemeColor(
					"statusBarItem.errorBackground",
				);
				break;
			}
			// @ts-expect-error Reserved for future use
			case Status.Warning: {
				this.statusBarItem.color = new ThemeColor(
					"statusBarItem.warningForeground",
				);
				this.statusBarItem.backgroundColor = new ThemeColor(
					"statusBarItem.warningBackground",
				);
				break;
			}
			default: {
				this.statusBarItem.color = undefined;
				this.statusBarItem.backgroundColor = undefined;
				break;
			}
		}

		this.statusBarItem.show();
	}

	public hide() {
		this.statusBarItem.hide();
	}
}
