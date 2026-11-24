// should generate diagnostics
export function read($props, $bindable) {
    const { unused = $bindable() } = $props();
}
