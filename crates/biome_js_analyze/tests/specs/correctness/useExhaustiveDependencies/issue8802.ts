import { useMemo } from "react";

export interface Days {
    mon: boolean;
    tue: boolean;
    wed: boolean;
    thu: boolean;
    fri: boolean;
}

const WeekdayValues: (keyof Days)[] = ["mon", "tue", "wed", "thu", "fri"];

// "day" doesn't exist outside the memoized function.
// Biome should report "props.value" as a missing dependency,
// NOT "props.value[day]" since "day" is a callback-scoped variable.
function Component(props: { value: Days }) {
    useMemo(() => {
        return WeekdayValues.filter((day) => props.value[day]);
    }, []);
}
