/* should not generate diagnostics */
/* does not generate diagnostics */

export function setUseUnreadActiveIcon(value: boolean): void {
    window.electron.tray.useUnreadActiveIcon(value);
}
