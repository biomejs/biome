// should not generate diagnostics

import { type FC, useEffect } from "react";

export const AnimatedNumber: FC = () => {
    useEffect(() => {
        let start: number | undefined;
        const step = (timestamp: number) => {
            if (start === undefined) {
                start = timestamp;
            }
        };
        console.log(step);
    }, []);
    return <div />;
};
