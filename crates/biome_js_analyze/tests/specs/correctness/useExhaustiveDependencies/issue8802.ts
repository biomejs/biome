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
function Component1(props: { value: Days }) {
    useMemo(() => {
        return WeekdayValues.filter((day) => props.value[day]);
    }, []);
}

// Example with forEach and index
// Biome should report "props.data" as missing dependency,
// NOT "props.data.forEach" to follow eslint React plugin behavior.
function Component2(props: { data: number[] }) {
    useMemo(() => {
        props.data.forEach((value, index) => {
            console.log(value, index);
        });
    }, []);
}
