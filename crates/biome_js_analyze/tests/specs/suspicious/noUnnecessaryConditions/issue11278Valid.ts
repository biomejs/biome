// should not generate diagnostics

export function capture(input: string): string | undefined {
    return /a(b)/.exec(input)?.[1];
}

export function captureDynamic(input: string, tag: string): string {
    return new RegExp(`<${tag}>([^<]*)`).exec(input)?.[1] ?? '';
}

export function captureCalled(input: string, tag: string): string {
    return RegExp(`<${tag}>([^<]*)`).exec(input)?.[1] ?? '';
}

export function matchIndex(input: string): number | undefined {
    return new RegExp('a(b)').exec(input)?.index;
}
