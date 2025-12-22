import { useEffect } from "react";

declare function useStable(): { stable: string };

export function SuccessComp() {
    const { stable } = useStable();

    useEffect(() => {
        console.log(stable);
    }, []);

    return null;
}

export function FailedComp() {
    const { stable: alias } = useStable();

    // This error is a false positive as the value is still stable
    useEffect(() => {
        console.log(alias);
    }, []);

    return null;
}
