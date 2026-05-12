/* should not generate diagnostics */

class IgnoredStringLike {
    value: string;
}

declare const ignored: IgnoredStringLike;

String(ignored);
ignored.toString();
`${ignored}`;
