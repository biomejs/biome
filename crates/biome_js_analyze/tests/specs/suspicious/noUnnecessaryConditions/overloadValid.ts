/* should not generate diagnostics */

interface Options {
    initial?: string;
}
interface DefinedOptions extends Options {
    initial: string;
}

type DefinedResult = { isPending: false; data: string };
type MaybeResult =
    | { isPending: true; data: undefined }
    | { isPending: false; data: string };

declare function query(options: DefinedOptions): DefinedResult;
declare function query(options: Options): MaybeResult;

export function queryState(flag: boolean) {
    const { isPending } = query({});
    return isPending || flag;
}
