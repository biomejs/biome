// should generate diagnostics

import { useCallback } from "react";

export type Props = { id: string };

export interface Handlers {
    onGo(): void;
}

export enum Mode {
    Default,
}

export function Component({ id }: Props) {
    const go = useCallback(() => console.log(id), []);
    return <button type="button" onClick={go}>go</button>;
}
