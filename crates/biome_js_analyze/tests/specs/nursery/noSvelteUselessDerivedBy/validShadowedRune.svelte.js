// should not generate diagnostics
export function select($derived) {
    return $derived.by(() => 1);
}
