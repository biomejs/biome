// should generate diagnostics

export function patternSource(pattern: string) {
    return new RegExp(pattern)?.source;
}

export function calledPatternSource(pattern: string) {
    return RegExp(pattern)?.source;
}
