import { useEffect, useState } from "react";
import type { SettingsTabProps } from "@/playground/tabs/SettingsTab";
import SettingsTab from "@/playground/tabs/SettingsTab";
import { classNames, createLocalStorage } from "@/playground/utils";

const isCollapsedStore = createLocalStorage("settings-collapsed");

export default function SettingsPanel(props: SettingsTabProps) {
	const [isCollapsed, setIsCollapsed] = useState(isCollapsedStore.getBoolean());

	function collapseToggle() {
		setIsCollapsed(!isCollapsed);
	}

	useEffect(() => {
		isCollapsedStore.set(isCollapsed);
	}, [isCollapsed]);

	return (
		<div className="settings-panel">
			{!isCollapsed && (
				<div className="fields">
					<SettingsTab {...props} />
				</div>
			)}
			<div
				className={classNames("collapser", isCollapsed && "collapsed")}
				onMouseDown={collapseToggle}
				onKeyDown={collapseToggle}
			>
				<div className="dot" />
				<div className="dot" />
				<div className="dot" />
			</div>
		</div>
	);
}
